use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#videonote
/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: i64,
    duration: i64,
    thumbnail: Option<PhotoSize>,
    file_size: Option<i64>,
}
