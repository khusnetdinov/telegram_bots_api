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
#[derive(Debug, Serialize, Deserialize)]
#[serde_with_macros::skip_serializing_none]
pub struct Update {
    update_id: i64,
    message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    message_reaction: Option<MessageReactionUpdated>,
    message_reaction_count: Option<MessageReactionCountUpdated>,
    inline_query: Option<InlineQuery>,
    chosen_inline_result: Option<ChosenInlineResult>,
    callback_query: Option<CallbackQuery>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
    poll: Option<PreCheckoutQuery>,
    poll_answer: Option<PollAnswer>,
    my_chat_member: Option<ChatMemberUpdated>,
    chat_member: Option<ChatMemberUpdated>,
    chat_join_request: Option<ChatJoinRequest>,
    chat_boost: Option<ChatBoostUpdated>,
    removed_chat_boost: Option<ChatBoostRemoved>,
}
