use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#reopenforumtopic>
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct ReopenForumTopic {
    pub chat_id: ChatUId,
    pub message_thread_id: i64,
}
