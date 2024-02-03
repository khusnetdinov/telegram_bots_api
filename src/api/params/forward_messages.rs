use serde::Serialize;

/// https://core.telegram.org/bots/api#forwardmessages
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct ForwardMessages {
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub from_chat_id: i64,
    pub message_ids: Vec<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
}
