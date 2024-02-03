use serde::Serialize;

/// https://core.telegram.org/bots/api#forwardmessage
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct ForwardMessage {
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub from_chat_id: i64,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_id: i64,
}
