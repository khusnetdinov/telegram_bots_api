use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#keyboardbuttonpolltype
/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeyboardButtonPollType {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: Option<String>,
}
