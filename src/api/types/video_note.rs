use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#videonote
/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<i64>,
}
