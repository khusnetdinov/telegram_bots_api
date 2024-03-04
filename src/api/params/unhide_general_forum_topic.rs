use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#unhidegeneralforumtopic
/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct UnhideGeneralForumTopic {
    pub chat_id: ChatUId,
}
