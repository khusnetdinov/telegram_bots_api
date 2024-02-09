use crate::api::types::order_info::OrderInfo;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#precheckoutquery
/// This object contains information about an incoming pre-checkout query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}
