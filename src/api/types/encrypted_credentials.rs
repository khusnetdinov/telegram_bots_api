use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#encryptedcredentials
/// Describes data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}
