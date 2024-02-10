use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageentity
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageEntity {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub offset: i64,
    pub length: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}
