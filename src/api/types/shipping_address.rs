use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#shippingaddress
/// This object represents a shipping address.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}
