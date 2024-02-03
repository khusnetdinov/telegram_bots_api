use serde::Serialize;

use crate::api::types::input_file::InputFile;

/// https://core.telegram.org/bots/api#setwebhook
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct SetWebhook {
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
    pub secret_token: Option<String>,
}
