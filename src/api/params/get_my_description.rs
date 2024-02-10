use serde::Serialize;

/// https://core.telegram.org/bots/api#getmydescription
/// Use this method to get the current bot description for the given user language. Returns BotDescription on success.
#[derive(Debug, Serialize)]
pub struct GetMyDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
