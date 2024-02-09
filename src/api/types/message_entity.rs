use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageentity
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageEntity {
    // type: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
    custom_emoji_id: Option<String>,
}
