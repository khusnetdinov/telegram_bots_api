use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerUser {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub user: User,
}
