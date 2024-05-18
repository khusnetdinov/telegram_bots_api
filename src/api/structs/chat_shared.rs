use crate::api::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatshared>
/// This object contains information about the chat whose identifier was shared with the bot using a KeyboardButtonRequestChat button.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatShared {
    pub request_id: i64,
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
