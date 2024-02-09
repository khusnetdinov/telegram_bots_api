use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorselfie
/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorSelfie {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
