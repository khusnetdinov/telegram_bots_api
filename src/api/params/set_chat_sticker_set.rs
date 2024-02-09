use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatstickerset
/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatStickerSet {
    chat_id: i64,
    sticker_set_name: String,
}
