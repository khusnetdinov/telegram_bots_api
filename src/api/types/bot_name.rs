use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botname
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotName {
    name: String,
}
