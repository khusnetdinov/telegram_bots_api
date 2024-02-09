use crate::api::types::mask_position::MaskPosition;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setstickermaskposition
/// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetStickerMaskPosition {
    sticker: String,
    mask_position: Option<MaskPosition>,
}
