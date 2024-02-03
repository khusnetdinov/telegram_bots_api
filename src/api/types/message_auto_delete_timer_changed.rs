use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#messageautodeletetimerchanged
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    message_auto_delete_time: i64,
}
