use crate::api::enums::revenue_withdrawal_state::RevenueWithdrawalState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerFragment {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
