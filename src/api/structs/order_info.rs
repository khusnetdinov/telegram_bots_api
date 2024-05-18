use crate::api::structs::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#orderinfo>
/// This object represents information about an order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
