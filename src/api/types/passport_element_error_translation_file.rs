use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrortranslationfile
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    source: String,
    // type: String,
    file_hash: String,
    message: String,
}
