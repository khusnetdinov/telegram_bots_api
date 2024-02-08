use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::chat_id::ChatId;
use crate::api::types::link_preview_options::LinkPreviewOptions;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::message_id::MessageId;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendmessage
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct SendMessage {
    pub chat_id: ChatId,
    pub message_thread_id: Option<MessageId>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
