use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#videochatparticipantsinvited
/// This object represents a service message about new members invited to a video chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}
