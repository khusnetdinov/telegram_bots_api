use crate::api::types::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#orderinfo
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}
