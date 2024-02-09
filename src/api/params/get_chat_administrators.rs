use serde::Serialize;

/// https://core.telegram.org/bots/api#getchatadministrators
/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
#[derive(Debug, Serialize)]
pub struct GetChatAdministrators {
    chat_id: i64,
}
