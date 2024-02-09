use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#botdescription
/// This object represents the bot's description.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BotDescription {
    description: String,
}
