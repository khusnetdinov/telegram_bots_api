use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#contact
/// This object represents a phone contact.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<i64>,
    vcard: Option<String>,
}
