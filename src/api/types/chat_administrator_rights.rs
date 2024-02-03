use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatadministratorrights
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
    is_anonymous: bool,
    can_manage_chat: bool,
    can_delete_messages: bool,
    can_manage_video_chats: bool,
    can_restrict_members: bool,
    can_promote_members: bool,
    can_change_info: bool,
    can_invite_users: bool,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_pin_messages: Option<bool>,
    can_post_stories: Option<bool>,
    can_edit_stories: Option<bool>,
    can_delete_stories: Option<bool>,
    can_manage_topics: Option<bool>,
}
