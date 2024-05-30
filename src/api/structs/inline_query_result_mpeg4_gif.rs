use crate::api::enums::input_message_content::InputMessageContent;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif>
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub mpeg4_url: String,
    pub thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}
