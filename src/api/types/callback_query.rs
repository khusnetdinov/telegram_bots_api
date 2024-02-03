use crate::api::types::maybe_inaccessible_message::MaybeInaccessibleMessage;
use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#callbackquery
#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackQuery {
    id: String,
    from: User,
    message: Option<MaybeInaccessibleMessage>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}
