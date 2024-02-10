use crate::api::types::callback_query::CallbackQuery;
use crate::api::types::chat_boost_removed::ChatBoostRemoved;
use crate::api::types::chat_boost_updated::ChatBoostUpdated;
use crate::api::types::chat_join_request::ChatJoinRequest;
use crate::api::types::chat_member_updated::ChatMemberUpdated;
use crate::api::types::chosen_inline_result::ChosenInlineResult;
use crate::api::types::inline_query::InlineQuery;
use crate::api::types::message::Message;
use crate::api::types::message_reaction_count_update::MessageReactionCountUpdated;
use crate::api::types::message_reaction_updated::MessageReactionUpdated;
use crate::api::types::poll_answer::PollAnswer;
use crate::api::types::pre_checkout_query::PreCheckoutQuery;
use crate::api::types::shipping_query::ShippingQuery;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#update
/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub message_reaction: Option<MessageReactionUpdated>,
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    pub poll: Option<PreCheckoutQuery>,
    pub poll_answer: Option<PollAnswer>,
    pub my_chat_member: Option<ChatMemberUpdated>,
    pub chat_member: Option<ChatMemberUpdated>,
    pub chat_join_request: Option<ChatJoinRequest>,
    pub chat_boost: Option<ChatBoostUpdated>,
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}
