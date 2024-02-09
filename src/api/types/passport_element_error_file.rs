use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorfile
/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFile {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
