use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::input_paid_media::InputPaidMedia;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::structs::message_entity::MessageEntity;
use crate::api::structs::reply_parameters::ReplyParameters;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#sendpaidmedia>
/// Use this method to send paid media to channel chats. On success, the sent Message is returned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendPaidMedia {
    pub chat_id: ChatUId,
    pub star_count: i64,
    pub media: Vec<InputPaidMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
}
