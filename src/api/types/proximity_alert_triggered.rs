use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#proximityalerttriggered
/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProximityAlertTriggered {
    traveler: User,
    watcher: User,
    distance: i64,
}
