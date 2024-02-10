use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forumtopiccreated
/// This object represents a service message about a new forum topic created in the chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}
