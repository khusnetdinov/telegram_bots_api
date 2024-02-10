use serde::Serialize;

/// https://core.telegram.org/bots/api#setmyname
/// Use this method to change the bot's name. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetMyName {
    pub name: Option<String>,
    pub language_code: Option<String>,
}
