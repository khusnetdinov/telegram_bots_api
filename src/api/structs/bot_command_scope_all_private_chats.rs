use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>
/// Represents the scope of bot commands, covering all private chats.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllPrivateChats {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
