use crate::api::structs::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#shippingoption>
/// This object represents one shipping option.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
