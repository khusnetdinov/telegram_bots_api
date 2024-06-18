use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#revenuewithdrawalstatefailed>
/// The withdrawal failed and the transaction was refunded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevenueWithdrawalStateFailed {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
