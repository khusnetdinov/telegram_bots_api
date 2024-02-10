use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::input_message_content::InputMessageContent;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultvideo
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVideo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumbnail_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub video_width: Option<i64>,
    pub video_height: Option<i64>,
    pub video_duration: Option<i64>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}
