use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatphoto>
/// This object represents a chat photo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}
