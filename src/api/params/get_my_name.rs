use serde::Serialize;

/// <https://core.telegram.org/bots/api#getmyname>
/// Use this method to get the current bot name for the given user language. Returns BotName on success.
#[derive(Debug, Serialize, Default)]
pub struct GetMyName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
