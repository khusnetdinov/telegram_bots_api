use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultvoice
/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVoice {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
