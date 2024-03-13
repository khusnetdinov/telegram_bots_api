use crate::api::enums::chat_uid::ChatUId;
use crate::api::structs::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#forwardmessages>
/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
#[derive(Debug, Serialize, Default)]
pub struct ForwardMessages {
    pub chat_id: ChatUId,
    pub from_chat_id: ChatUId,
    pub message_ids: Vec<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}
