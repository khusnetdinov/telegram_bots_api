use crate::api::types::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputinvoicemessagecontent
/// Represents the content of an invoice message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputInvoiceMessageContent {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub max_tip_amount: Option<i64>,
    pub suggested_tip_amounts: Option<Vec<i64>>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i64>,
    pub photo_width: Option<i64>,
    pub photo_height: Option<i64>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
}
