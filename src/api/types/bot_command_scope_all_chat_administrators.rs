use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopeallchatadministrators
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    // type: String,
}
