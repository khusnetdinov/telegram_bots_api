use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrorfrontside
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
