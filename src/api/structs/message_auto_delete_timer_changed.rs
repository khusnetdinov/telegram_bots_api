use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messageautodeletetimerchanged>
/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}
