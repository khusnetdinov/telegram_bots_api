use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#contact>
/// This object represents a phone contact.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
