use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botname
#[derive(Debug, Serialize, Deserialize)]
pub struct BotName {
    name: String,
}
