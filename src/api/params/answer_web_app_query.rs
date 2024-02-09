use crate::api::types::inline_query_result::InlineQueryResult;
use serde::Serialize;

/// https://core.telegram.org/bots/api#answerwebappquery
/// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
#[derive(Debug, Serialize)]
struct AnswerWebAppQuery {
    web_app_query_id: String,
    result: InlineQueryResult,
}
