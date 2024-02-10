use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendgame
/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendGame {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub game_short_name: String,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
