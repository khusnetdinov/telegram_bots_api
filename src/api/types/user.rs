use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#user
/// This object represents a Telegram user or bot.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde_with_macros::skip_serializing_none]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
}
