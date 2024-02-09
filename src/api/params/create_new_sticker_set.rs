use crate::api::types::input_sticker::InputSticker;
use serde::Serialize;

/// https://core.telegram.org/bots/api#createnewstickerset
#[derive(Debug, Serialize)]
struct CreateNewStickerSet {
    user_id: i64,
    name: String,
    title: String,
    stickers: Vec<InputSticker>,
    sticker_format: String,
    sticker_type: Option<String>,
    needs_repainting: Option<bool>,
}
