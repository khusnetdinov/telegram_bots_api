use serde::Serialize;

/// https://core.telegram.org/bots/api#getmyshortdescription
/// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
#[derive(Debug, Serialize, Default)]
pub struct GetMyShortDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
