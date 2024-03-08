use crate::api::types::mask_position::MaskPosition;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#setstickermaskposition>
/// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetStickerMaskPosition {
    pub sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}
