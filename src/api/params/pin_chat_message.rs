use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#pinchatmessage>
/// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct PinChatMessage {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}
