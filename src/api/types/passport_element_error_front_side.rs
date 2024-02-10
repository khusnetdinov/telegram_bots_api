use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorfrontside
/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub file_hash: String,
    pub source: String,
    pub message: String,
}
