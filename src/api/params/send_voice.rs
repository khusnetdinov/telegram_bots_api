use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendvoice
/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Debug, Serialize)]
pub struct SendVoice {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub voice: FileInput,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
