use crate::api::structs::business_connection::BusinessConnection;
use crate::api::structs::business_message_deleted::BusinessMessagesDeleted;
use crate::api::structs::callback_query::CallbackQuery;
use crate::api::structs::chat_boost_removed::ChatBoostRemoved;
use crate::api::structs::chat_boost_updated::ChatBoostUpdated;
use crate::api::structs::chat_join_request::ChatJoinRequest;
use crate::api::structs::chat_member_updated::ChatMemberUpdated;
use crate::api::structs::chosen_inline_result::ChosenInlineResult;
use crate::api::structs::inline_query::InlineQuery;
use crate::api::structs::message::Message;
use crate::api::structs::message_reaction_count_update::MessageReactionCountUpdated;
use crate::api::structs::message_reaction_updated::MessageReactionUpdated;
use crate::api::structs::poll::Poll;
use crate::api::structs::poll_answer::PollAnswer;
use crate::api::structs::pre_checkout_query::PreCheckoutQuery;
use crate::api::structs::purchased_paid_media::PaidMediaPurchased;
use crate::api::structs::shipping_query::ShippingQuery;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#update>
/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub update_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<PaidMediaPurchased>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<BusinessConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,
}
