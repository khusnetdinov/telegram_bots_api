use serde::Serialize;

/// https://core.telegram.org/bots/api#createforumtopic
/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns information about the created topic as a ForumTopic object.
#[derive(Debug, Serialize)]
pub struct CreateForumTopic {
    chat_id: i64,
    name: String,
    icon_color: Option<i64>,
    icon_custom_emoji_id: Option<String>,
}
