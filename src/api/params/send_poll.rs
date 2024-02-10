use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendpoll
/// Use this method to send a native poll. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendPoll {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: Option<String>,
    pub allows_multiple_answers: Option<bool>,
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub question: String,
    pub options: Vec<String>,
    pub is_anonymous: Option<bool>,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
