use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#webappdata>
/// Describes data sent from a Web App to the bot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
