use crate::api::structs::animation::Animation;
use crate::api::structs::message_entity::MessageEntity;
use crate::api::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#game>
/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}
