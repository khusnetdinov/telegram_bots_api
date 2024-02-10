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
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}
