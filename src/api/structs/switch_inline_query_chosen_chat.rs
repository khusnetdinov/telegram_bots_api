use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#switchinlinequerychosenchat>
/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SwitchInlineQueryChosenChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}
