use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#switchinlinequerychosenchat
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SwitchInlineQueryChosenChat {
    query: Option<String>,
    allow_user_chats: Option<bool>,
    allow_bot_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
}
