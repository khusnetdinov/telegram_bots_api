use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#webappdata
/// Describes data sent from a Web App to the bot.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebAppData {
    data: String,
    button_text: String,
}
