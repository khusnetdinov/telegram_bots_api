use serde::Serialize;

/// https://core.telegram.org/bots/api#setstickerkeywords
/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetStickerKeywords {
    sticker: String,
    keywords: Option<Vec<String>>,
}
