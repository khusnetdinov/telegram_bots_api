use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#stoppoll>
/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
#[derive(Debug, Serialize, Default)]
pub struct StopPoll {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
