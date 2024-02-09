use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#giveawaycompleted
/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompleted {
    winner_count: i64,
    unclaimed_prize_count: Option<i64>,
    giveaway_completed: Option<Box<GiveawayCompleted>>,
}
