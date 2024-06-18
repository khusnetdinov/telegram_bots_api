use crate::api::structs::revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed;
use crate::api::structs::revenue_withdrawal_state_pending::RevenueWithdrawalStatePending;
use crate::api::structs::revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#revenuewithdrawalstate>
/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
///
/// RevenueWithdrawalStatePending
/// RevenueWithdrawalStateSucceeded
/// RevenueWithdrawalStateFailed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}
