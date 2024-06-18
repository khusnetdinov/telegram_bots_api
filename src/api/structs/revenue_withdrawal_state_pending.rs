use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#revenuewithdrawalstatepending>
/// The withdrawal is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevenueWithdrawalStatePending {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
