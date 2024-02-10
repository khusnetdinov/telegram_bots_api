use crate::api::types::file::File;
use crate::api::types::mask_position::MaskPosition;
use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#sticker
/// This object represents a sticker.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub width: i64,
    pub file_id: String,
    pub file_unique_id: String,
    pub height: i64,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<i64>,
}
