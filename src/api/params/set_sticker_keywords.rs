use serde::Serialize;

/// https://core.telegram.org/bots/api#setstickerkeywords
/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetStickerKeywords {
    pub sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}
