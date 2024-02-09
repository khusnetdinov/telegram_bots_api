use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#photosize
/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: i64,
    height: i64,
    file_size: Option<i64>,
}
