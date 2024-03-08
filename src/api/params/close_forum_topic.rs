use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#closeforumtopic>
/// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct CloseForumTopic {
    pub chat_id: ChatUId,
    pub message_thread_id: i64,
}
