use serde::Serialize;

/// https://core.telegram.org/bots/api#deletewebhook
#[derive(Debug, Serialize, Default)]
#[serde_with_macros::skip_serializing_none]
pub struct DeleteWebhook {
    pub drop_pending_updates: Option<bool>,
}
