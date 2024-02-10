use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#getchatmembercount
/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Debug, Serialize)]
pub struct GetChatMemberCount {
    pub chat_id: ChatUId,
}
