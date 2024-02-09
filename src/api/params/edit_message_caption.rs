use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::message_entity::MessageEntity;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editmessagecaption
/// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize)]
pub struct EditMessageCaption {
    chat_id: Option<ChatUId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    caption_entities: Option<Vec<MessageEntity>>,
    reply_markup: Option<InlineKeyboardMarkup>,
}
