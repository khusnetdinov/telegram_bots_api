use crate::api::structs::transaction_partner_fragment::TransactionPartnerFragment;
use crate::api::structs::transaction_partner_other::TransactionPartnerOther;
use crate::api::structs::transaction_partner_user::TransactionPartnerUser;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#transactionpartner>
/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
///
/// TransactionPartnerFragment
/// TransactionPartnerUser
/// TransactionPartnerOther
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionPartner {
    Fragment(TransactionPartnerFragment),
    User(TransactionPartnerUser),
    Other(TransactionPartnerOther),
}
