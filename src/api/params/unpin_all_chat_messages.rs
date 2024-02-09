use serde::Serialize;

/// https://core.telegram.org/bots/api#unpinallchatmessages
///
#[derive(Debug, Serialize)]
pub struct UnpinAllChatMessages {
    chat_id: i64,
}
