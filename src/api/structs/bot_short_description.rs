use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#botshortdescription>
/// This object represents the bot's short description.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotShortDescription {
    pub short_description: String,
}
