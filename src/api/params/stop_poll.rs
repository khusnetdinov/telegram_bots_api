use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#stoppoll
/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
#[derive(Debug, Serialize)]
pub struct StopPoll {
    pub chat_id: ChatUId,
    pub message_id: MessageId,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
