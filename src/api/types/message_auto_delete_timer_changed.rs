use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageautodeletetimerchanged
/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: i64,
}
