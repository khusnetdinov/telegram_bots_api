use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorselfie
/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorSelfie {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub file_hash: String,
    pub source: String,
    pub message: String,
}
