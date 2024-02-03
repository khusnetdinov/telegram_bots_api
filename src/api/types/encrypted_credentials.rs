use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#encryptedcredentials
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}
