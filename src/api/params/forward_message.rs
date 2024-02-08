use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#forwardmessage
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct ForwardMessage {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<MessageId>,
    pub from_chat_id: ChatUId,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_id: MessageId,
}
