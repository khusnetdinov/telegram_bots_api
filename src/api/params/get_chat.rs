use serde::Serialize;

/// https://core.telegram.org/bots/api#getchat
/// Use this method to get up to date information about the chat. Returns a Chat object on success.
#[derive(Debug, Serialize)]
pub struct GetChat {
    chat_id: i64,
}
