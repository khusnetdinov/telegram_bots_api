use crate::api::types::labeled_price::LabeledPrice;

/// https://core.telegram.org/bots/api#createinvoicelink
/// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
struct CreateInvoiceLink {
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    max_tip_amount: Option<i64>,
    suggested_tip_amounts: Option<Vec<i64>>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
}
