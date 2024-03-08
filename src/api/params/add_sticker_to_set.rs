use crate::api::types::input_sticker::InputSticker;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#addstickertoset>
/// Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct AddStickerToSet {
    pub user_id: i64,
    pub name: String,
    pub sticker: InputSticker,
}
