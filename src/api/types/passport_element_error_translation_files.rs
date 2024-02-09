use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrortranslationfiles
/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFiles {
    source: String,
    // type: String,
    file_hashes: Vec<String>,
    message: String,
}
