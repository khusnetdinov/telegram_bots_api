use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#invoice
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i64,
}
