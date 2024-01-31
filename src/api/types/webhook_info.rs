use serde::Deserialize;

/// https://core.telegram.org/bots/api#webhookinfo
/// Describes the current status of a webhook.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct WebhookInfo {
    url: String,
    has_custom_certificate: bool,
    pending_update_count: i64,
    ip_address: Option<String>,
    last_error_date: Option<i64>,
    last_error_message: Option<String>,
    last_synchronization_error_date: Option<i64>,
    max_connections: Option<i64>,
    allowed_updates: Option<Vec<String>>,
}
