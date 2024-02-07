use crate::api::types::file::File;
use crate::api::types::mask_position::MaskPosition;
use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#sticker
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    file_id: String,
    file_unique_id: String,
    // type: String,
    width: i64,
    height: i64,
    is_animated: bool,
    is_video: bool,
    thumbnail: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    premium_animation: Option<File>,
    mask_position: Option<MaskPosition>,
    custom_emoji_id: Option<String>,
    needs_repainting: Option<bool>,
    file_size: Option<i64>,
}
