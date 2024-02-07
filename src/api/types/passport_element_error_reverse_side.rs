use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrorreverseside
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorReverseSide {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
