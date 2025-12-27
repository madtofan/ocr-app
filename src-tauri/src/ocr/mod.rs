pub mod manga_ocr;
pub mod pp_ocr;

use crate::{models::OcrBox, state::AppState, Result};
use image::DynamicImage;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

// In your Tauri setup / initialization
pub fn init_ort(app_handle: &AppHandle) -> Result<()> {
    // --- 1. RESOLVE THE LIBRARY PATH ---
    // Only do this once if possible, or check if already initialized
    let resource_path = app_handle
        .path()
        .resolve("resources/onnx-libs", BaseDirectory::Resource)
        .expect("failed to resolve resource path");

    // Determine the filename based on the current OS
    let lib_name = if cfg!(target_os = "windows") {
        "onnxruntime.dll"
    } else if cfg!(target_os = "macos") {
        "libonnxruntime.dylib"
    } else {
        "libonnxruntime.so"
    };

    let lib_path = resource_path.join(lib_name);

    // Verify the file actually exists (good for debugging)
    if !lib_path.exists() {
        return Err(crate::Error::Other(format!(
            "Critical Error: ONNX Library not found at path: {:?}",
            lib_path
        )));
    }

    // --- 2. TELL 'ORT' WHERE TO LOOK ---
    // We set the environment variable that 'ort' checks during init
    let _ = std::env::set_var("ORT_DYLIB_PATH", lib_path.to_str().unwrap());

    // --- 3. INITIALIZE ORT ---
    ort::init().with_name("Manga-OCR").commit()?;
    Ok(())
}

pub async fn run_ocr(app: &AppHandle, full_image: DynamicImage) -> Result<Vec<OcrBox>> {
    // 1. Acquire the locks
    let state = app.state::<AppState>();
    let mut det_session = state.det_session.lock().unwrap();
    let mut enc_session = state.enc_session.lock().unwrap();
    let mut dec_session = state.dec_session.lock().unwrap();
    let tokenizer = state.tokenizer.lock().unwrap();

    // --- 2. DETECTION (PP-OCRv5 Mobile Det) ---

    // **--- PRE-PROCESSING STEP ---**
    let preprocessed_image = pp_ocr::preprocess_image(&full_image);

    // --- 2. DETECTION (PP-OCRv5 Mobile Det) ---
    let detected_boxes = pp_ocr::detect(&mut det_session, &preprocessed_image)?;

    // --- 3. RECOGNITION (Manga-OCR Encoder-Decoder) ---
    let mut final_results = Vec::new();

    for mut bbox in detected_boxes {
        // a. Crop the image from the original screenshot
        let cropped_image: image::DynamicImage = full_image.clone().crop_imm(
            bbox.x as u32,
            bbox.y as u32,
            bbox.width as u32,
            bbox.height as u32,
        );

        // d. Decoder Loop (Autoregressive)
        let recognized_text = manga_ocr::recognize(
            &mut enc_session,
            &mut dec_session,
            &tokenizer,
            &cropped_image,
        )?;

        bbox.text = recognized_text;
        final_results.push(bbox);
    }

    Ok(final_results)
}
