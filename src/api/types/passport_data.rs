use crate::api::types::encrypted_credentials::EncryptedCredentials;
use crate::api::types::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#passportdata>
/// Describes Telegram Passport data shared with the bot by the user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
