use serde::Serialize;

/// https://core.telegram.org/bots/api#reopenforumtopic
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
#[derive(Debug, Serialize)]
pub struct ReopenForumTopic {
    chat_id: i64,
    message_thread_id: i64,
}
