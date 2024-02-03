use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#keyboardbuttonpolltype
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    // type: Option<String>,
}
