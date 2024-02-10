use crate::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use crate::api::enums::message_origin::MessageOrigin;
use crate::api::types::animation::Animation;
use crate::api::types::audio::Audio;
use crate::api::types::chat::Chat;
use crate::api::types::chat_shared::ChatShared;
use crate::api::types::contact::Contact;
use crate::api::types::dice::Dice;
use crate::api::types::document::Document;
use crate::api::types::external_reply_info::ExternalReplyInfo;
use crate::api::types::forum_topic_closed::ForumTopicClosed;
use crate::api::types::forum_topic_created::ForumTopicCreated;
use crate::api::types::forum_topic_edited::ForumTopicEdited;
use crate::api::types::forum_topic_reopened::ForumTopicReopened;
use crate::api::types::game::Game;
use crate::api::types::general_forum_topic_hidden::GeneralForumTopicHidden;
use crate::api::types::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use crate::api::types::giveaway::Giveaway;
use crate::api::types::giveaway_completed::GiveawayCompleted;
use crate::api::types::giveaway_created::GiveawayCreated;
use crate::api::types::giveaway_winners::GiveawayWinners;
use crate::api::types::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::api::types::invoice::Invoice;
use crate::api::types::link_preview_options::LinkPreviewOptions;
use crate::api::types::location::Location;
use crate::api::types::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use crate::api::types::message_entity::MessageEntity;
use crate::api::types::message_id::MessageId;
use crate::api::types::passport_data::PassportData;
use crate::api::types::photo_size::PhotoSize;
use crate::api::types::poll::Poll;
use crate::api::types::proximity_alert_triggered::ProximityAlertTriggered;
use crate::api::types::sticker::Sticker;
use crate::api::types::story::Story;
use crate::api::types::successful_payment::SuccessfulPayment;
use crate::api::types::text_quote::TextQuote;
use crate::api::types::user::User;
use crate::api::types::users_shared::UsersShared;
use crate::api::types::venue::Venue;
use crate::api::types::video::Video;
use crate::api::types::video_chat_ended::VideoChatEnded;
use crate::api::types::video_chat_participants_invited::VideoChatParticipantsInvited;
use crate::api::types::video_chat_scheduled::VideoChatScheduled;
use crate::api::types::video_chat_started::VideoChatStarted;
use crate::api::types::video_note::VideoNote;
use crate::api::types::voice::Voice;
use crate::api::types::web_app_data::WebAppData;
use crate::api::types::write_access_allowed::WriteAccessAllowed;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#message
/// This object represents a message.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Message {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub message_thread_id: Option<i64>,
    pub from: Option<User>,
    pub sender_chat: Option<Box<Chat>>,
    pub date: i64,
    pub chat: Chat,
    pub forward_origin: Option<MessageOrigin>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub external_reply: Option<ExternalReplyInfo>,
    pub quote: Option<TextQuote>,
    pub via_bot: Option<User>,
    pub edit_date: Option<i64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub users_shared: Option<UsersShared>,
    pub chat_shared: Option<ChatShared>,
    pub connected_website: Option<String>,
    pub write_access_allowed: Option<WriteAccessAllowed>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub forum_topic_created: Option<ForumTopicCreated>,
    pub forum_topic_edited: Option<ForumTopicEdited>,
    pub forum_topic_closed: Option<ForumTopicClosed>,
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    pub giveaway_created: Option<GiveawayCreated>,
    pub giveaway: Option<Giveaway>,
    pub giveaway_winners: Option<GiveawayWinners>,
    pub giveaway_completed: Option<GiveawayCompleted>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
