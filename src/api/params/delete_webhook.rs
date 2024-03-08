use serde::Serialize;

/// <https://core.telegram.org/bots/api#deletewebhook>
/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct DeleteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}
