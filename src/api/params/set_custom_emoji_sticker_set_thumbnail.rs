use serde::Serialize;

/// https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
/// Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetCustomEmojiStickerSetThumbnail {
    pub name: String,
    pub custom_emoji_id: Option<String>,
}
