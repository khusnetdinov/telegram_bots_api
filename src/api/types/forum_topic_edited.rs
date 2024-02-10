use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forumtopicedited
/// This object represents a service message about an edited forum topic.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}
