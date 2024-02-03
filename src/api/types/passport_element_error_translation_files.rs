use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrortranslationfiles
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    source: String,
    // type: String,
    file_hashes: Vec<String>,
    message: String,
}
