use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#voice
/// This object represents a voice note.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}
