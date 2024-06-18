use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#revenuewithdrawalstatesucceeded>
/// The withdrawal succeeded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevenueWithdrawalStateSucceeded {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub url: String,
}
