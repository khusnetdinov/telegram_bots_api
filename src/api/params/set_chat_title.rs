use serde::Serialize;

/// https://core.telegram.org/bots/api#setchattitle
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatTitle {
    chat_id: i64,
    title: String,
}
