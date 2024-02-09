use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// https://core.telegram.org/bots/api#stoppoll
/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
#[derive(Debug, Serialize)]
pub struct StopPoll {
    chat_id: ChatUId,
    message_id: i64,
    reply_markup: Option<InlineKeyboardMarkup>,
}
