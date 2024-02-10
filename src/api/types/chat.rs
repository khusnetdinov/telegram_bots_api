use crate::api::enums::reaction_type::ReactionType;
use crate::api::types::chat_location::ChatLocation;
use crate::api::types::chat_permissions::ChatPermissions;
use crate::api::types::chat_photo::ChatPhoto;
use crate::api::types::message::Message;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chat
/// This object represents a chat.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Chat {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: i64,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub active_usernames: Option<Vec<String>>,
    pub available_reactions: Option<Vec<ReactionType>>,
    pub accent_color_id: Option<i64>,
    pub background_custom_emoji_id: Option<String>,
    pub profile_accent_color_id: Option<i64>,
    pub profile_background_custom_emoji_id: Option<String>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub emoji_status_expiration_date: Option<i64>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<bool>,
    pub has_restricted_voice_and_video_messages: Option<bool>,
    pub join_to_send_messages: Option<bool>,
    pub join_by_request: Option<bool>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<i64>,
    pub message_auto_delete_time: Option<i64>,
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    pub has_hidden_members: Option<bool>,
    pub has_protected_content: Option<bool>,
    pub has_visible_history: Option<bool>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>,
}
