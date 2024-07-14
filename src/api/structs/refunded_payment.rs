use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#refundedpayment>
/// This object contains basic information about a refunded payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}
