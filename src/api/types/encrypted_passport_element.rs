use crate::api::types::passport_file::PassportFile;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#encryptedpassportelement
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EncryptedPassportElement {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}
