use serde::Serialize;

/// <https://core.telegram.org/bots/api#getcustomemojistickers>
/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
#[derive(Debug, Serialize, Default)]
pub struct GetCustomEmojiStickers {
    pub custom_emoji_ids: Vec<String>,
}
