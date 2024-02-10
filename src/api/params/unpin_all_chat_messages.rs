use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#unpinallchatmessages
///
#[derive(Debug, Serialize)]
pub struct UnpinAllChatMessages {
    pub chat_id: ChatUId,
}
