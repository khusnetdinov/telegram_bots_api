use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatphoto
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatPhoto {
    small_file_id: String,
    small_file_unique_id: String,
    big_file_id: String,
    big_file_unique_id: String,
}
