use crate::api::structs::chat::Chat;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#businessmessagesdeleted>
/// This object is received when messages are deleted from a connected business account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i32>,
}
