use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrorfile
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
