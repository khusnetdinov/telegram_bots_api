use serde::Serialize;

/// https://core.telegram.org/bots/api#deletewebhook
/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct DeleteWebhook {
    pub drop_pending_updates: Option<bool>,
}
