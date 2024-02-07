use crate::api::types::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#shippingoption
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}
