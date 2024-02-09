use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#audio
/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
    thumbnail: Option<PhotoSize>,
}
