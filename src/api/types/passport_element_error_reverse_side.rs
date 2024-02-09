use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorreverseside
/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorReverseSide {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
