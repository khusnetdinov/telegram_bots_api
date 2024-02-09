use serde::Serialize;

/// https://core.telegram.org/bots/api#setstickersettitle
/// Use this method to set the title of a created sticker set. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetStickerSetTitle {
    name: String,
    title: String,
}
