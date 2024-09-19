use crate::api::enums::chat_uid::ChatUId;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::structs::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#editmessagereplymarkup>
/// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize, Default)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatUId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
}
