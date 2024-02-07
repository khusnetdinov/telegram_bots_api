use crate::api::types::animation::Animation;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#game
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}
