use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#transactionpartnerother>
/// Describes a transaction with an unknown source or recipient.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerOther {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
