use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendvideonote
/// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendVideoNote {
    chat_id: i64,
    message_thread_id: Option<i64>,
    // video_note: InputFile or String,
    duration: Option<i64>,
    length: Option<i64>,
    // thumbnail: Option<InputFile or String>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
    reply_markup: Option<ReplyMarkup>,
}
