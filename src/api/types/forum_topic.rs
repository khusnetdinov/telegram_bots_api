use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forumtopic
/// This object represents a forum topic.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopic {
    message_thread_id: i64,
    name: String,
    icon_color: i64,
    icon_custom_emoji_id: Option<String>,
}
