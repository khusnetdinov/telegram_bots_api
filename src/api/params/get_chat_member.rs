use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#getchatmember
/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a ChatMember object on success.
#[derive(Debug, Serialize)]
pub struct GetChatMember {
    chat_id: ChatUId,
    user_id: i64,
}
