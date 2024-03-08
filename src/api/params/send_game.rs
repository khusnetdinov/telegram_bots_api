use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#sendgame>
/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Debug, Serialize, Default)]
pub struct SendGame {
    pub chat_id: ChatUId,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
