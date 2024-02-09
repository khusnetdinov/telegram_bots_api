use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#switchinlinequerychosenchat
/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SwitchInlineQueryChosenChat {
    query: Option<String>,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}
