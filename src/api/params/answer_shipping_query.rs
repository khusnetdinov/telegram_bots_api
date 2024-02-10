use crate::api::types::shipping_option::ShippingOption;
use serde::Serialize;

/// https://core.telegram.org/bots/api#answershippingquery
/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
#[derive(Debug, Serialize)]
pub struct AnswerShippingQuery {
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>,
}
