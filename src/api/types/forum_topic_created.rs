use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forumtopiccreated
/// This object represents a service message about a new forum topic created in the chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicCreated {
    name: String,
    icon_color: i64,
    icon_custom_emoji_id: Option<String>,
}
