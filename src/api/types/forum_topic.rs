use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#forumtopic
#[derive(Debug, Serialize, Deserialize)]
pub struct ForumTopic {
    message_thread_id: i64,
    name: String,
    icon_color: i64,
    icon_custom_emoji_id: Option<String>,
}
