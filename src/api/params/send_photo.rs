use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendphoto
/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendPhoto {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub photo: FileInput,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_spoiler: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
