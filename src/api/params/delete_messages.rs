use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#deletemessages
/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct DeleteMessages {
    pub chat_id: ChatUId,
    pub message_ids: Vec<i64>,
}
