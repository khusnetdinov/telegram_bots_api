use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chat>
/// This object represents a chat.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Chat {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
}
