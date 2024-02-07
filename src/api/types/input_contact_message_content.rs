use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputcontactmessagecontent
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
}
