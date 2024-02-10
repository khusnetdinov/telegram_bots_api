use crate::api::enums::inline_query_result::InlineQueryResult;
use crate::api::types::inline_query_results_button::InlineQueryResultsButton;
use serde::Serialize;

/// https://core.telegram.org/bots/api#answerinlinequery
/// Use this method to send answers to an inline query. On success, True is returned.
/// No more than 50 results per query are allowed.
#[derive(Debug, Serialize)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}
