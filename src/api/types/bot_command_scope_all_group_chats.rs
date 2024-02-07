use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botcommandscopeallgroupchats
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCommandScopeAllGroupChats {
    // type: String,
}
