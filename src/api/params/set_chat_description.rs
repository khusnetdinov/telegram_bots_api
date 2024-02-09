use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatdescription
/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatDescription {
    chat_id: i64,
    description: Option<String>,
}
