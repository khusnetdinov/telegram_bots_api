use crate::api::enums::revenue_withdrawal_state::RevenueWithdrawalState;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#transactionpartnerfragment>
/// Describes a withdrawal transaction with Fragment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerFragment {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
