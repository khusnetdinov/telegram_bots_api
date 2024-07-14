use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#transactionpartnertelegramads>
/// Describes a withdrawal transaction to the Telegram Ads platform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPartnerTelegramAds {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
