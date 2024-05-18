use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#passportelementerrorfile>
/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFile {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub file_hash: String,
    pub message: String,
}
