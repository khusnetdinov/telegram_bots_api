use serde::Serialize;

/// <https://core.telegram.org/bots/api#setmyname>
/// Use this method to change the bot's name. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetMyName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
