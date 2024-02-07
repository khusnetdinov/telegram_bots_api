use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultaudio
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultAudio {
    // type: String,
    id: String,
    audio_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    performer: Option<String>,
    audio_duration: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}
