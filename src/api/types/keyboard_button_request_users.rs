use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#keyboardbuttonrequestusers
/// This object defines the criteria used to request suitable users. The identifiers of the selected users will be shared with the bot when the corresponding button is pressed.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonRequestUsers {
    request_id: i64,
    user_is_bot: Option<bool>,
    user_is_premium: Option<bool>,
    max_quantity: Option<i64>,
}
