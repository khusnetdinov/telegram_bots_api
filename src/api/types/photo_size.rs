use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#photosize
/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}
