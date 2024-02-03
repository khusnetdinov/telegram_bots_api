use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#botdescription
#[derive(Debug, Serialize, Deserialize)]
pub struct BotDescription {
    description: String,
}
