use serde::Serialize;

/// https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
/// Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetCustomEmojiStickerSetThumbnail {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}
