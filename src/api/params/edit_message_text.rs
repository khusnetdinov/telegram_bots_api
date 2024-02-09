use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::link_preview_options::LinkPreviewOptions;
use crate::api::types::message_entity::MessageEntity;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editmessagetext
/// Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize)]
pub struct EditMessageText {
    chat_id: Option<ChatUId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    link_preview_options: Option<LinkPreviewOptions>,
    reply_markup: Option<InlineKeyboardMarkup>,
}
