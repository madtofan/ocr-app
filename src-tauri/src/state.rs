use ort::session::Session;
use sqlx::{Pool, Sqlite};
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use tokenizers::Tokenizer;

pub struct AppState {
    pub db: Pool<Sqlite>,
    pub is_processing: Arc<AtomicBool>,
    pub det_session: Mutex<Session>,
    pub enc_session: Mutex<Session>,
    pub dec_session: Mutex<Session>,
    pub tokenizer: Mutex<Tokenizer>,
}
