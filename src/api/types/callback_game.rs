use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#callbackgame
/// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CallbackGame {}
