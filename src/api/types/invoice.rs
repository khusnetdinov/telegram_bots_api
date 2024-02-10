use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#invoice
/// This object contains basic information about an invoice.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}
