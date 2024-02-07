use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#voice
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: i64,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
