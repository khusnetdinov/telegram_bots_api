use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#video
/// This object represents a video file.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Video {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
