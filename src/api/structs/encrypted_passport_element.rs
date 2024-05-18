use crate::api::structs::passport_file::PassportFile;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#encryptedpassportelement>
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedPassportElement {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}
