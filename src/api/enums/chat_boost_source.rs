use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboostsource
/// This object describes the source of a chat boost. It can be one of
/// ChatBoostSourcePremium
/// ChatBoostSourceGiftCode
/// ChatBoostSourceGiveaway
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSource {}
