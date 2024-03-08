use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#hidegeneralforumtopic>
/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct HideGeneralForumTopic {
    pub chat_id: ChatUId,
}
