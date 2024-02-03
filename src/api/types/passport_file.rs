use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportfile
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportFile {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    file_date: i64,
}
