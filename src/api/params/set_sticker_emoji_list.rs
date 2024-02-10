use serde::Serialize;

/// https://core.telegram.org/bots/api#setstickeremojilist
/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetStickerEmojiList {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}
