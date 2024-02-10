use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportfile
/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_date: i64,
}
