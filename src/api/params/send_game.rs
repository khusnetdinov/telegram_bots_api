use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendgame
/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendGame {
    chat_id: ChatUId,
    message_thread_id: Option<i64>,
    game_short_name: String,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<InlineKeyboardMarkup>,
}
