use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#contact
/// This object represents a phone contact.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}
