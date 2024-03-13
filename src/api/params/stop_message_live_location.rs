use crate::api::enums::chat_uid::ChatUId;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#stopmessagelivelocation>
/// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize, Default)]
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatUId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
