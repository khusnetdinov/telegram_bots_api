use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botdescription>
/// This object represents the bot's description.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotDescription {
    pub description: String,
}
