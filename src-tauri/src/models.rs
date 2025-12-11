use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: Option<i64>, // Optional because new items don't have an ID yet
    title: String,
    status: String,
    created_at: String, // Simplified as string for this demo
}
