use crate::api::structs::shipping_address::ShippingAddress;
use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#shippingquery>
/// This object contains information about an incoming shipping query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
