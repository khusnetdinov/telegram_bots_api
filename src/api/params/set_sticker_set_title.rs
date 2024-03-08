use serde::Serialize;

/// <https://core.telegram.org/bots/api#setstickersettitle>
/// Use this method to set the title of a created sticker set. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetStickerSetTitle {
    pub name: String,
    pub title: String,
}
