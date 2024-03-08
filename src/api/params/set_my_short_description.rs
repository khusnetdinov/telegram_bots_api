use serde::Serialize;

/// <https://core.telegram.org/bots/api#setmyshortdescription>
/// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetMyShortDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
