use crate::api::structs::input_sticker::InputSticker;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#createnewstickerset>
#[derive(Debug, Serialize, Default)]
pub struct CreateNewStickerSet {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}
