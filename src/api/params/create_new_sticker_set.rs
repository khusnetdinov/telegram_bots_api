use crate::api::types::input_sticker::InputSticker;
use serde::Serialize;

/// https://core.telegram.org/bots/api#createnewstickerset
#[derive(Debug, Serialize)]
struct CreateNewStickerSet {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    pub sticker_format: String,
    pub sticker_type: Option<String>,
    pub needs_repainting: Option<bool>,
}
