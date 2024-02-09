use serde::Serialize;

/// https://core.telegram.org/bots/api#deletemessages
/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
#[derive(Debug, Serialize)]
struct DeleteMessages {
    chat_id: i64,
    message_ids: Vec<i64>,
}
