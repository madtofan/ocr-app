use crate::{models::OcrBox, Result};
use image::{imageops, DynamicImage, GrayImage};
use imageproc::filter;
use ndarray::ArrayD;
use ort::{session::Session, value::Value};

pub fn preprocess_image(input_image: &DynamicImage) -> GrayImage {
    // 1. cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    let gray_image: GrayImage = input_image.to_luma8();

    // 2. cv2.GaussianBlur(gray, (5, 5), 0)
    let gray_f32: image::ImageBuffer<image::Luma<f32>, Vec<f32>> =
        image::ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
            let pixel = gray_image.get_pixel(x, y);
            image::Luma([pixel[0] as f32])
        });

    let blurred_f32 = filter::gaussian_blur_f32(&gray_f32, 1.0);

    // 3. cv2.threshold(blur, 128, 255, cv2.THRESH_BINARY_INV)
    let threshold = 128.0;
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

pub fn detect(det_session: &mut Session, preprocessed_image: &GrayImage) -> Result<Vec<OcrBox>> {
    // The detection model requires input dimensions to be multiples of 32.
    // We pad the image to the nearest multiple of 32.
    let (h, w) = (preprocessed_image.height(), preprocessed_image.width());
    let new_h = ((h + 31) / 32) * 32;
    let new_w = ((w + 31) / 32) * 32;

    let mut padded_image = image::GrayImage::new(new_w, new_h);
    imageops::overlay(&mut padded_image, preprocessed_image, 0, 0);

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
    let _det_outputs = det_session.run(ort::inputs![
        "x" => &input_value
    ])?;

    // c. Post-processing (The most complex part!)
    let detected_boxes: Vec<OcrBox> = {
        // This is where you implement DB-Net post-processing in Rust
        // to convert the det_outputs probability map into actual bounding boxes.
        // Returning a placeholder for now:
        vec![OcrBox {
            text: "".to_string(),
            x: 50,
            y: 50,
            width: 200,
            height: 40,
        }]
    };

    Ok(detected_boxes)
}
