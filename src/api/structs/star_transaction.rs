use crate::api::enums::transaction_partner::TransactionPartner;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#startransaction>
/// Describes a Telegram Star transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarTransaction {
    pub id: i64,
    pub amount: u64,
    pub date: i64,
    pub source: Option<TransactionPartner>,
    pub receiver: Option<TransactionPartner>,
}
