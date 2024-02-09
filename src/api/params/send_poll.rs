use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendpoll
/// Use this method to send a native poll. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendPoll {
    chat_id: i64,
    message_thread_id: Option<i64>,
    question: String,
    options: Vec<String>,
    is_anonymous: Option<bool>,
    // type: Option<String>,
    allows_multiple_answers: Option<bool>,
    correct_option_id: Option<i64>,
    explanation: Option<String>,
    explanation_parse_mode: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<i64>,
    close_date: Option<i64>,
    is_closed: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<ReplyMarkup>,
}
