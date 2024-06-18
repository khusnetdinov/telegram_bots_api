use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerOther {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
