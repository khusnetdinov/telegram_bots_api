use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#forumtopicedited
#[derive(Debug, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    name: Option<String>,
    icon_custom_emoji_id: Option<String>,
}
