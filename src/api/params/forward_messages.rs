use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#forwardmessages
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct ForwardMessages {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<MessageId>,
    pub from_chat_id: ChatUId,
    pub message_ids: Vec<MessageId>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
}
