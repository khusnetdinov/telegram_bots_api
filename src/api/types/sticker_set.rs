use crate::api::types::photo_size::PhotoSize;
use crate::api::types::sticker::Sticker;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#stickerset
/// This object represents a sticker set.
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
