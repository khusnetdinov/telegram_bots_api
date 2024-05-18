use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botcommandscopedefault>
/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeDefault {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
