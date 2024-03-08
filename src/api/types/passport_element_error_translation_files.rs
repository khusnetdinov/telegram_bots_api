use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#passportelementerrortranslationfiles>
/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub file_hashes: Vec<String>,
    pub message: String,
}
