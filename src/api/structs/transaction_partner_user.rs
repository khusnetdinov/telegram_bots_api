use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#transactionpartneruser
/// Describes a transaction with a user.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerUser {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
}
