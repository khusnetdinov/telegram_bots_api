use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#paidmediapreview>
/// The paid media isn't available before the payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaidMediaPreview {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
