use crate::api::enums::input_message_content::InputMessageContent;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i64>,
    pub mpeg4_height: Option<i64>,
    pub mpeg4_duration: Option<i64>,
    pub thumbnail_url: String,
    pub thumbnail_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
