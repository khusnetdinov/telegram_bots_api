use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrortranslationfile
/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFile {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
