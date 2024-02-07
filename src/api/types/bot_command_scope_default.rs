use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopedefault
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeDefault {
    // type: String,
}
