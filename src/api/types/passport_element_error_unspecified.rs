use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorunspecified
/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorUnspecified {
    source: String,
    // type: String,
    element_hash: String,
    message: String,
}
