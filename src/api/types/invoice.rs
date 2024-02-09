use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#invoice
/// This object contains basic information about an invoice.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i64,
}
