use crate::api::types::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#orderinfo
/// This object represents information about an order.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}
