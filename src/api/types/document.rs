use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#document
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    file_id: String,
    file_unique_id: String,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
