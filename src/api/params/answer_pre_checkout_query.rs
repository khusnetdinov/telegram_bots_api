use serde::Serialize;

/// https://core.telegram.org/bots/api#answerprecheckoutquery
/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(Debug, Serialize, Default)]
pub struct AnswerPreCheckoutQuery {
    pub ok: bool,
    pub pre_checkout_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
