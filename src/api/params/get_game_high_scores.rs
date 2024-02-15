use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#getgamehighscores
/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
/// This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
#[derive(Debug, Serialize)]
pub struct GetGameHighScores {
    user_id: i64,
    chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}
