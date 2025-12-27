use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrBox {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub created_at: String,
}