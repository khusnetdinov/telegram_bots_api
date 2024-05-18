use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#labeledprice>
/// This object represents a portion of the price for goods or services.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}
