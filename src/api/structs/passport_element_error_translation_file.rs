use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#passportelementerrortranslationfile>
/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub file_hash: String,
    pub source: String,
    pub message: String,
}
