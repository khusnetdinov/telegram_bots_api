use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#videochatparticipantsinvited
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    users: Vec<User>,
}