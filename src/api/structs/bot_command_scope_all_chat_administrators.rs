use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>
/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllChatAdministrators {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
