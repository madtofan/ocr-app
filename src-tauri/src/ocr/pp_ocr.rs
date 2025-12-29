use crate::{models::OcrBox, ocr::image_buffer_to_base64, Result};
use image::{
    imageops::{self, FilterType},
    DynamicImage, GrayImage,
};
use imageproc::contours::find_contours;
use imageproc::filter;
use ndarray::ArrayD;
use ort::{session::Session, value::Value};
use tauri::{AppHandle, Emitter};

const PADDING_MULTIPLIER: f64 = 2.2;
const LINE_WIDTH_MULTIPLIER: u32 = 2;
const PADDLE_OCR_MULTIPLIER: u32 = 32;
const MAX_RESOLUTION: u32 = 1920;

pub fn preprocess_image(input_image: &DynamicImage) -> GrayImage {
    // Resizing to make it run faster
    let resized_image = input_image.resize(MAX_RESOLUTION, MAX_RESOLUTION, FilterType::CatmullRom);

    // 1. cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    let gray_image: GrayImage = resized_image.to_luma8();

    // 2. cv2.GaussianBlur(gray, (5, 5), 0)
    let gray_f32: image::ImageBuffer<image::Luma<f32>, Vec<f32>> =
        image::ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
            let pixel = gray_image.get_pixel(x, y);
            image::Luma([pixel[0] as f32])
        });

    let blurred_f32 = filter::gaussian_blur_f32(&gray_f32, 1.0);

    // 3. cv2.threshold(blur, 153, 255, cv2.THRESH_BINARY_INV)
    let threshold = 153.0;
    let binary_image: image::GrayImage =
        image::ImageBuffer::from_fn(blurred_f32.width(), blurred_f32.height(), |x, y| {
            let pixel = blurred_f32.get_pixel(x, y);
            if pixel[0] > threshold {
                image::Luma([0u8]) // Black Background
            } else {
                image::Luma([255u8]) // White Text
            }
        });

    binary_image
}

pub fn detect(
    app: &AppHandle,
    det_session: &mut Session,
    preprocessed_image: &GrayImage,
    full_image: &DynamicImage,
) -> Result<Vec<OcrBox>> {
    let (h_full, w_full) = (full_image.height(), full_image.width());

    let mut base64_images = Vec::new();
    // The detection model requires input dimensions to be multiples of 32.
    // We pad the image to the nearest multiple of 32.
    let (h, w) = (preprocessed_image.height(), preprocessed_image.width());
    let new_h = ((h + PADDLE_OCR_MULTIPLIER - 31) / PADDLE_OCR_MULTIPLIER) * PADDLE_OCR_MULTIPLIER;
    let new_w = ((w + PADDLE_OCR_MULTIPLIER - 31) / PADDLE_OCR_MULTIPLIER) * PADDLE_OCR_MULTIPLIER;

    let multiplier = if h > w {
        h_full as f64 / h as f64
    } else {
        w_full as f64 / w as f64
    };

    let mut padded_image = image::GrayImage::new(new_w, new_h);
    imageops::overlay(&mut padded_image, preprocessed_image, 0, 0);

    base64_images.push(image_buffer_to_base64(padded_image.clone().into()).unwrap());

    // a. Convert the GrayImage (u8) into the required ONNX input tensor (f32, often normalized).
    let (h, w) = (
        padded_image.height() as usize,
        padded_image.width() as usize,
    );
    let single_channel_data: Vec<f32> =
        padded_image.pixels().map(|p| p[0] as f32 / 255.0).collect();

    // The model expects a 3-channel image (RGB), but we have a grayscale image.
    // We can replicate the single channel data three times to create a 3-channel image.
    // The ONNX model expects data in NCHW format (batch, channels, height, width).
    let mut chw_data: Vec<f32> = Vec::with_capacity(3 * h * w);
    chw_data.extend_from_slice(&single_channel_data); // R channel
    chw_data.extend_from_slice(&single_channel_data); // G channel
    chw_data.extend_from_slice(&single_channel_data); // B channel

    // Fix: Use IxDyn for ndarray 0.17 compatibility
    let shape = ndarray::IxDyn(&[1, 3, h, w]);
    let det_input_array =
        ArrayD::from_shape_vec(shape, chw_data).map_err(|e| crate::Error::Other(e.to_string()))?;

    let input_value = Value::from_array(det_input_array)?;

    // b. Run detection session
    let det_outputs = det_session.run(ort::inputs![
        "x" => &input_value
    ])?;

    // PaddleOCR detection models usually output a tensor of shape [1, 1, H, W]
    let (shape, heatmap) = det_outputs[0]
        .try_extract_tensor::<f32>()
        .map_err(|e| e.to_string())?;

    let height = shape[2] as usize;
    let width = shape[3] as usize;
    let threshold = 0.3; // Standard for PaddleOCR
    let mut binary_mask: image::GrayImage = image::ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let score = heatmap[y * width + x];
            let val = if score > threshold { 255u8 } else { 0u8 };
            binary_mask.put_pixel(x as u32, y as u32, image::Luma([val]));
        }
    }

    base64_images.push(image_buffer_to_base64(binary_mask.clone().into()).unwrap());

    // Find clusters of white pixels
    let contours = find_contours::<i32>(&binary_mask);

    let mut detected_boxes = Vec::new();

    for contour in contours {
        // 1. Calculate the bounding box for this contour
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for pt in contour.points {
            min_x = min_x.min(pt.x);
            max_x = max_x.max(pt.x);
            min_y = min_y.min(pt.y);
            max_y = max_y.max(pt.y);
        }

        min_x = ((min_x as f64) * multiplier).ceil() as i32;
        max_x = ((max_x as f64) * multiplier).ceil() as i32;
        min_y = ((min_y as f64) * multiplier).ceil() as i32;
        max_y = ((max_y as f64) * multiplier).ceil() as i32;

        let box_width = (max_x - min_x) as u32;
        let box_height = (max_y - min_y) as u32;

        if box_width > 5 && box_height > 5 {
            if box_height > box_width {
                let pixel_padding =
                    ((box_width as f64 * PADDING_MULTIPLIER).ceil() as u32) - box_width;
                detected_boxes.push(OcrBox {
                    x: min_x as u32 - (pixel_padding / LINE_WIDTH_MULTIPLIER),
                    y: min_y as u32 - pixel_padding,
                    width: box_width + pixel_padding,
                    height: box_height + (pixel_padding * LINE_WIDTH_MULTIPLIER),
                    text: String::new(),
                });
            } else {
                let pixel_padding =
                    ((box_height as f64 * PADDING_MULTIPLIER).ceil() as u32) - box_height;
                detected_boxes.push(OcrBox {
                    x: min_x as u32 - pixel_padding,
                    y: min_y as u32 - (pixel_padding / LINE_WIDTH_MULTIPLIER),
                    width: box_width + (pixel_padding * LINE_WIDTH_MULTIPLIER),
                    height: box_height + pixel_padding,
                    text: String::new(),
                });
            }
        }
    }

    app.emit("base64-images2", base64_images).unwrap();

    Ok(detected_boxes)
}
