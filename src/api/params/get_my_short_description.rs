use serde::Serialize;

/// https://core.telegram.org/bots/api#getmyshortdescription
/// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
#[derive(Debug, Serialize)]
pub struct GetMyShortDescription {
    pub language_code: Option<String>,
}
