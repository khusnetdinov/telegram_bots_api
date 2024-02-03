use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrorselfie
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
