use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopedefault
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {
    // type: String,
}
