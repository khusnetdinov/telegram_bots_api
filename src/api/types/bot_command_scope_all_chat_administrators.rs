use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscopeallchatadministrators
/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllChatAdministrators {
    // type: String,
}
