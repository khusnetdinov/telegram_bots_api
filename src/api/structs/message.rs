use crate::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use crate::api::enums::message_origin::MessageOrigin;
use crate::api::structs::animation::Animation;
use crate::api::structs::audio::Audio;
use crate::api::structs::chat::Chat;
use crate::api::structs::chat_boost_added::ChatBoostAdded;
use crate::api::structs::chat_shared::ChatShared;
use crate::api::structs::contact::Contact;
use crate::api::structs::dice::Dice;
use crate::api::structs::document::Document;
use crate::api::structs::external_reply_info::ExternalReplyInfo;
use crate::api::structs::forum_topic_closed::ForumTopicClosed;
use crate::api::structs::forum_topic_created::ForumTopicCreated;
use crate::api::structs::forum_topic_edited::ForumTopicEdited;
use crate::api::structs::forum_topic_reopened::ForumTopicReopened;
use crate::api::structs::game::Game;
use crate::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden;
use crate::api::structs::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use crate::api::structs::giveaway::Giveaway;
use crate::api::structs::giveaway_completed::GiveawayCompleted;
use crate::api::structs::giveaway_created::GiveawayCreated;
use crate::api::structs::giveaway_winners::GiveawayWinners;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::structs::invoice::Invoice;
use crate::api::structs::link_preview_options::LinkPreviewOptions;
use crate::api::structs::location::Location;
use crate::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use crate::api::structs::message_entity::MessageEntity;
use crate::api::structs::message_id::MessageId;
use crate::api::structs::passport_data::PassportData;
use crate::api::structs::photo_size::PhotoSize;
use crate::api::structs::poll::Poll;
use crate::api::structs::proximity_alert_triggered::ProximityAlertTriggered;
use crate::api::structs::sticker::Sticker;
use crate::api::structs::story::Story;
use crate::api::structs::successful_payment::SuccessfulPayment;
use crate::api::structs::text_quote::TextQuote;
use crate::api::structs::user::User;
use crate::api::structs::users_shared::UsersShared;
use crate::api::structs::venue::Venue;
use crate::api::structs::video::Video;
use crate::api::structs::video_chat_ended::VideoChatEnded;
use crate::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited;
use crate::api::structs::video_chat_scheduled::VideoChatScheduled;
use crate::api::structs::video_chat_started::VideoChatStarted;
use crate::api::structs::video_note::VideoNote;
use crate::api::structs::voice::Voice;
use crate::api::structs::web_app_data::WebAppData;
use crate::api::structs::write_access_allowed::WriteAccessAllowed;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#message>
/// This object represents a message.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Message {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub date: i64,
    pub chat: Chat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<Chat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<UsersShared>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<Box<ChatBoostAdded>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Box<Story>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i64>,
}
