use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputcontactmessagecontent
/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
