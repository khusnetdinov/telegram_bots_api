use crate::api::types::inline_query_result::InlineQueryResult;
use crate::api::types::inline_query_results_button::InlineQueryResultsButton;
use serde::Serialize;

/// https://core.telegram.org/bots/api#answerinlinequery
/// Use this method to send answers to an inline query. On success, True is returned.
/// No more than 50 results per query are allowed.
#[derive(Debug, Serialize)]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    cache_time: Option<i64>,
    is_personal: Option<bool>,
    next_offset: Option<String>,
    button: Option<InlineQueryResultsButton>,
}
