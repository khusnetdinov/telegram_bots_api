use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendvideonote
/// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendVideoNote {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub video_note: FileInput,
    pub duration: Option<i64>,
    pub length: Option<i64>,
    pub thumbnail: Option<FileInput>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
