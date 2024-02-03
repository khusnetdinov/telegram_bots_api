use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#proximityalerttriggered
#[derive(Debug, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: i64,
}
