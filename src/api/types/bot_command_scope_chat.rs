use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopechat
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommandScopeChat {
    // type: String,
    chat_id: i64,
}
