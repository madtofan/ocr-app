use crate::Result;
use image::DynamicImage;
use ndarray::{Array2, Array4};
use ort::{
    session::Session,
    value::{Tensor, Value},
};
use tokenizers::Tokenizer;

// These constants are typical for Vision Encoder-Decoder models like Manga-OCR (TrOCR).
// Verify these IDs against your specific tokenizer.json file if possible.
const MAX_LENGTH: usize = 100;
const START_TOKEN_ID: i64 = 2; // Typically 2 for <SOS>
const END_TOKEN_ID: i64 = 2; // Often the same ID is used for <EOS> (2)

// Constants for Manga-OCR (adjust size if your specific model uses 384)
const TARGET_SIZE: u32 = 224;
const MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const STD: [f32; 3] = [0.229, 0.224, 0.225];

pub fn recognize(
    enc_session: &mut Session,
    dec_session: &mut Session,
    tokenizer: &Tokenizer,
    cropped_image: &DynamicImage,
) -> Result<String> {
    // b. Pre-process the crop for the encoder (e.g., resize to 224x224, normalize)
    let enc_input_tensor: Tensor<f32> = {
        // 1. Resize the crop to the model's required input size
        let resized = cropped_image.resize_exact(
            TARGET_SIZE,
            TARGET_SIZE,
            image::imageops::FilterType::Triangle,
        );
        let rgb_img = resized.to_rgb8();
        // 2. Convert to ndarray [Batch, Channel, Height, Width]
        let mut input_array =
            Array4::<f32>::zeros((1, 3, TARGET_SIZE as usize, TARGET_SIZE as usize));

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            for c in 0..3 {
                // Normalize: (pixel_value / 255.0 - mean) / std
                let val = (pixel[c] as f32 / 255.0 - MEAN[c]) / STD[c];
                input_array[[0, c, y as usize, x as usize]] = val;
            }
        }

        Value::from_array(input_array.into_dyn())?
    };

    // c. Encoder Pass (One-time)
    let enc_outputs = enc_session
        .run(ort::inputs!["pixel_values" => enc_input_tensor])?;

    let hidden_state_value = &enc_outputs["last_hidden_state"];

    // d. Decoder Loop (Autoregressive)
    run_autoregressive_decoding(dec_session, tokenizer, hidden_state_value)
}

fn run_autoregressive_decoding(
    dec_session: &mut Session,
    tokenizer: &Tokenizer,
    encoder_hidden_state: &Value,
) -> Result<String> {
    // 1. Initialization
    let mut current_ids = vec![START_TOKEN_ID];

    for _ in 0..MAX_LENGTH {
        // Prepare input sequence: shape [1, seq_len]
        let seq_len = current_ids.len();
        let input_ids = Array2::from_shape_vec((1, seq_len), current_ids.clone())
            .map_err(|e| crate::Error::Other(e.to_string()))?
            .into_dyn(); // Convert to ArrayD to match model expectations

        let input_ids_value = Value::from_array(input_ids)?;
        let outputs = dec_session
            .run(ort::inputs![
                "encoder_hidden_states" => encoder_hidden_state,
                "input_ids" => input_ids_value,
            ])?;

        // Extract and Decode
        let (shape, raw_data) = outputs["logits"]
            .try_extract_tensor::<f32>()?;

        let vocab_size = shape[2] as usize;

        let last_token_index = seq_len - 1;
        let start_offset = last_token_index * vocab_size;
        let end_offset = start_offset + vocab_size;

        let next_id = raw_data[start_offset..end_offset]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index as i64)
            .unwrap();

        if next_id == END_TOKEN_ID {
            break;
        }
        current_ids.push(next_id);
    }

    // Decode (skip start token)
    let final_ids: Vec<u32> = current_ids[1..].iter().map(|&id| id as u32).collect();
    tokenizer
        .decode(&final_ids, true)
        .map_err(|e| crate::Error::Tokenizer(e.to_string()))
}
