use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    // type: String,
    id: String,
    mpeg4_url: String,
    mpeg4_width: Option<i64>,
    mpeg4_height: Option<i64>,
    mpeg4_duration: Option<i64>,
    thumbnail_url: String,
    thumbnail_mime_type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}
