use crate::api::types::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#shippingoption
/// This object represents one shipping option.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}
