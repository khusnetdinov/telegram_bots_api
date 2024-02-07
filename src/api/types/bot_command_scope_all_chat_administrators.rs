use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopeallchatadministrators
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllChatAdministrators {
    // type: String,
}
