use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrorfiles
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    source: String,
    // type: String,
    file_hashes: Vec<String>,
    message: String,
}
