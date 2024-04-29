use crate::api::structs::input_sticker::InputSticker;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#replacestickerinset>
/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling deleteStickerFromSet, then addStickerToSet, then setStickerPositionInSet. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct ReplaceStickerInSet {
    pub user_id: i64,
    pub name: String,
    pub old_sticker: String,
    pub sticker: InputSticker,
}
