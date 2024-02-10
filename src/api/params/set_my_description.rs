use serde::Serialize;

/// https://core.telegram.org/bots/api#setmydescription
/// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success.
#[derive(Debug, Serialize)]
struct SetMyDescription {
    pub description: Option<String>,
    pub language_code: Option<String>,
}
