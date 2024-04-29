use crate::api::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#shareduser>
/// This object contains information about a user that was shared with the bot using a KeyboardButtonRequestUser button.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SharedUser {
    pub user_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
