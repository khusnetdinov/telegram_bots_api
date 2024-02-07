use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrorunspecified
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorUnspecified {
    source: String,
    // type: String,
    element_hash: String,
    message: String,
}
