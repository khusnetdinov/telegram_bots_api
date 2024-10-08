use crate::api::enums::chat_uid::ChatUId;
use crate::api::structs::message_id::MessageId;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#getgamehighscores>
/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
/// This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetGameHighScores {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatUId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
