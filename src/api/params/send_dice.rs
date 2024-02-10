use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#senddice
/// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendDice {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
