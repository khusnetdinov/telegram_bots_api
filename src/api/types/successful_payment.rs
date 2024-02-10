use crate::api::types::order_info::OrderInfo;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#successfulpayment
/// This object contains basic information about a successful payment.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}
