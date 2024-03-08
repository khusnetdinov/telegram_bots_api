use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;
/// <https://core.telegram.org/bots/api#createforumtopic>
/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns information about the created topic as a ForumTopic object.
#[derive(Debug, Serialize, Default)]
pub struct CreateForumTopic {
    pub chat_id: ChatUId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}
