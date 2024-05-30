use serde::Serialize;

///<https://core.telegram.org/bots/api#refundstarpayment>
/// Refunds a successful payment in Telegram Stars. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct RefundStarPayment {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
}
