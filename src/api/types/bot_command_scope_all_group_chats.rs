use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopeallgroupchats
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    // type: String,
}
