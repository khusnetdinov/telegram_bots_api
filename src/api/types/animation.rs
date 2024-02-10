use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#animation
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
