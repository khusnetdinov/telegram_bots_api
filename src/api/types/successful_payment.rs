use crate::api::types::order_info::OrderInfo;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#successfulpayment
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}