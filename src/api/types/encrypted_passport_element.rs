use crate::api::types::passport_file::PassportFile;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#encryptedpassportelement
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EncryptedPassportElement {
    // type: String,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}
