use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendanimation
/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, Serialize)]
pub struct SendAnimation {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub animation: FileInput,
    pub duration: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub thumbnail: Option<FileInput>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_spoiler: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
