use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopeallgroupchats>
/// Represents the scope of bot commands, covering all group and supergroup chats.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllGroupChats {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
