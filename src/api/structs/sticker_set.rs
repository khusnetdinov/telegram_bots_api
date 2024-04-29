use crate::api::structs::photo_size::PhotoSize;
use crate::api::structs::sticker::Sticker;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#stickerset>
/// This object represents a sticker set.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: String,
    pub stickers: Vec<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}
