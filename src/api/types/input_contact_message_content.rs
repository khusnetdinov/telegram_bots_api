use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputcontactmessagecontent
/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputContactMessageContent {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}
