use serde::Serialize;

/// https://core.telegram.org/bots/api#pinchatmessage
/// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
#[derive(Debug, Serialize)]
pub struct PinChatMessage {
    chat_id: i64,
    message_id: i64,
    disable_notification: Option<bool>,
}
