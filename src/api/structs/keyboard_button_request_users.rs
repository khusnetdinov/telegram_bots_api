use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#keyboardbuttonrequestusers>
/// This object defines the criteria used to request suitable users. The identifiers of the selected users will be shared with the bot when the corresponding button is pressed.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}
