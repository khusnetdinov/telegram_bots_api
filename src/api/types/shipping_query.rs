use crate::api::types::shipping_address::ShippingAddress;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#shippingquery
/// This object contains information about an incoming shipping query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}
