use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorfrontside
/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFrontSide {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
