use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;
/// https://core.telegram.org/bots/api#deleteforumtopic
/// Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct DeleteForumTopic {
    pub chat_id: ChatUId,
    pub message_thread_id: i64,
}
