use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#forwardmessage>
/// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent Message is returned.
#[derive(Debug, Serialize, Default)]
pub struct ForwardMessage {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub chat_id: ChatUId,
    pub from_chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}
