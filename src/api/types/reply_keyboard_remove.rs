use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#replykeyboardremove
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    selective: Option<bool>,
}
