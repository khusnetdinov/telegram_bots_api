use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#giveawaycompleted>
/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<GiveawayCompleted>>,
}
