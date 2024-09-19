use crate::api::enums::chat_uid::ChatUId;
use crate::api::structs::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#setgamescore>
/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[derive(Debug, Serialize, Default)]
pub struct SetGameScore {
    pub user_id: i64,
    pub score: u64,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<ChatUId>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
}
