use crate::api::enums::input_message_content::InputMessageContent;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inlinequeryresultcachedaudio>
/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InlineQueryResultCachedAudio {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub audio_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
