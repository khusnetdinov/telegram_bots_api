use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#user>
/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde_with_macros::skip_serializing_none]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub an_connect_to_business: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
}
