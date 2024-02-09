use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#labeledprice
/// This object represents a portion of the price for goods or services.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LabeledPrice {
    label: String,
    amount: i64,
}
