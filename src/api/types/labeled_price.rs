use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#labeledprice
#[derive(Debug, Serialize, Deserialize)]
pub struct LabeledPrice {
    label: String,
    amount: i64,
}
