use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#shippingaddress
#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}
