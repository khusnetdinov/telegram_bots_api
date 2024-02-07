use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botdescription
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotDescription {
    description: String,
}
