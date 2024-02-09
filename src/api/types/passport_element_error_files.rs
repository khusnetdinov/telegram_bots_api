use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrorfiles
/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFiles {
    source: String,
    // type: String,
    file_hashes: Vec<String>,
    message: String,
}
