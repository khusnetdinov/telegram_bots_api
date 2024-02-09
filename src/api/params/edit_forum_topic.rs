use serde::Serialize;

/// https://core.telegram.org/bots/api#editforumtopic
/// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
#[derive(Debug, Serialize)]
pub struct EditForumTopic {
    chat_id: i64,
    message_thread_id: i64,
    name: Option<String>,
    icon_custom_emoji_id: Option<String>,
}
