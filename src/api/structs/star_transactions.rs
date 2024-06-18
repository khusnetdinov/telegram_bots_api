use crate::api::structs::star_transaction::StarTransaction;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#startransactions>
/// Contains a list of Telegram Star transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}
