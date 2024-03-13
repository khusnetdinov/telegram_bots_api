use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#forumtopicedited>
/// This object represents a service message about an edited forum topic.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}
