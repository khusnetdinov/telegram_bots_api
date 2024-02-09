use serde::Serialize;

use crate::api::types::input_file::InputFile;

/// https://core.telegram.org/bots/api#setwebhook
/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
/// If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
#[derive(Debug, Serialize, Default)]
pub struct SetWebhook {
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
    pub secret_token: Option<String>,
}
