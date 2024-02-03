use crate::api::types::chat_location::ChatLocation;
use crate::api::types::chat_permissions::ChatPermissions;
use crate::api::types::chat_photo::ChatPhoto;
use crate::api::types::message::Message;
use crate::api::types::reaction_type::ReactionType;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chat
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Chat {
    id: i64,
    // type: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_forum: Option<bool>,
    photo: Option<ChatPhoto>,
    active_usernames: Option<Vec<String>>,
    available_reactions: Option<Vec<ReactionType>>,
    accent_color_id: Option<i64>,
    background_custom_emoji_id: Option<String>,
    profile_accent_color_id: Option<i64>,
    profile_background_custom_emoji_id: Option<String>,
    emoji_status_custom_emoji_id: Option<String>,
    emoji_status_expiration_date: Option<i64>,
    bio: Option<String>,
    has_private_forwards: Option<bool>,
    has_restricted_voice_and_video_messages: Option<bool>,
    join_to_send_messages: Option<bool>,
    join_by_request: Option<bool>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Box<Message>>,
    permissions: Option<ChatPermissions>,
    slow_mode_delay: Option<i64>,
    message_auto_delete_time: Option<i64>,
    has_aggressive_anti_spam_enabled: Option<bool>,
    has_hidden_members: Option<bool>,
    has_protected_content: Option<bool>,
    has_visible_history: Option<bool>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
    linked_chat_id: Option<i64>,
    location: Option<ChatLocation>,
}
