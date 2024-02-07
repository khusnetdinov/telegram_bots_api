use crate::api::types::photo_size::PhotoSize;
use crate::api::types::sticker::Sticker;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#stickerset
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StickerSet {
    name: String,
    title: String,
    sticker_type: String,
    is_animated: bool,
    is_video: bool,
    stickers: Vec<Sticker>,
    thumbnail: Option<PhotoSize>,
}
