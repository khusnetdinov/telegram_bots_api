use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#senddice
/// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendDice {
    chat_id: ChatUId,
    message_thread_id: Option<i64>,
    emoji: Option<String>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<ReplyMarkup>,
}
