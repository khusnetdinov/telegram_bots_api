use crate::api::types::animation::Animation;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#game
/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}
