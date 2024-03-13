use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopechat>
/// Represents the scope of bot commands, covering a specific chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeChat {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub chat_id: i64,
}
