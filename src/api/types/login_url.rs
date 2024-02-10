use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#loginurl
/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginUrl {
    url: String,
    forward_text: Option<String>,
    bot_username: Option<String>,
    request_write_access: Option<bool>,
}
