use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botcommandscopedefault
/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeDefault {
    // type: String,
}
