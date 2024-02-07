use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LabeledPrice {
    label: String,
    amount: i64,
}
