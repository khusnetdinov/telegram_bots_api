use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#unbanchatsenderchat
/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct UnbanChatSenderChat {
    pub chat_id: ChatUId,
    pub sender_chat_id: ChatUId,
}
