use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editmessagereplymarkup
/// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    pub chat_id: Option<ChatUId>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
