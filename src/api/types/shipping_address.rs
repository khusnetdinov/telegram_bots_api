use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#shippingaddress
/// This object represents a shipping address.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}
