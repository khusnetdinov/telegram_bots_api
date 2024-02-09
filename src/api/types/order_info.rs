use crate::api::types::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#orderinfo
/// This object represents information about an order.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}
