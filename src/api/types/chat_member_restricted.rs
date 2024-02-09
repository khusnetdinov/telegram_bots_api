use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatmemberrestricted
/// Represents a chat member that is under certain restrictions in the chat. Supergroups only.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberRestricted {
    status: String,
    user: User,
    is_member: bool,
    can_send_messages: bool,
    can_send_audios: bool,
    can_send_documents: bool,
    can_send_photos: bool,
    can_send_videos: bool,
    can_send_video_notes: bool,
    can_send_voice_notes: bool,
    can_send_polls: bool,
    can_send_other_messages: bool,
    can_add_web_page_previews: bool,
    can_change_info: bool,
    can_invite_users: bool,
    can_pin_messages: bool,
    can_manage_topics: bool,
    until_date: i64,
}
