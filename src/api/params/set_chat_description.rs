use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatdescription
/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetChatDescription {
    pub chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
