use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videonote
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_size: Option<i64>,
}
