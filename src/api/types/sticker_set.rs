use crate::api::types::photo_size::PhotoSize;
use crate::api::types::sticker::Sticker;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#stickerset
/// This object represents a sticker set.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: String,
    pub is_animated: bool,
    pub is_video: bool,
    pub stickers: Vec<Sticker>,
    pub thumbnail: Option<PhotoSize>,
}
