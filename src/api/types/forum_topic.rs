use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forumtopic
/// This object represents a forum topic.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: i64,
    pub icon_custom_emoji_id: Option<String>,
}
