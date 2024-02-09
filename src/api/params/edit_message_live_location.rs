use crate::api::enums::chat_uid::ChatUId;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// https://core.telegram.org/bots/api#editmessagelivelocation
/// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize)]
struct EditMessageLiveLocation {
    chat_id: Option<ChatUId>,
    message_id: Option<i64>,
    inline_message_id: Option<String>,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: Option<f64>,
    heading: Option<i64>,
    proximity_alert_radius: Option<i64>,
    reply_markup: Option<InlineKeyboardMarkup>,
}
