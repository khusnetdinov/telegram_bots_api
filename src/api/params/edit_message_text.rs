use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::link_preview_options::LinkPreviewOptions;
use crate::api::types::message_entity::MessageEntity;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editmessagetext
/// Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize, Default)]
pub struct EditMessageText {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatUId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
