use serde::Serialize;

/// https://core.telegram.org/bots/api#getmyname
/// Use this method to get the current bot name for the given user language. Returns BotName on success.
#[derive(Debug, Serialize)]
pub struct GetMyName {
    language_code: Option<String>,
}
