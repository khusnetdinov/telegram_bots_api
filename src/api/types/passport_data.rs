use crate::api::types::encrypted_credentials::EncryptedCredentials;
use crate::api::types::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportdata
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}
