use crate::api::types::message::Message;
use crate::api::types::{
    CallbackQuery, ChatBoostRemoved, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated,
    ChosenInlineResult, InlineQuery, MessageReactionCountUpdated, MessageReactionUpdated, Poll,
    PollAnswer, PreCheckoutQuery, ShippingQuery,
};
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
    poll: Option<Poll>,
    poll_answer: Option<PollAnswer>,
    my_chat_member: Option<ChatMemberUpdated>,
    chat_member: Option<ChatMemberUpdated>,
    chat_join_request: Option<ChatJoinRequest>,
    chat_boost: Option<ChatBoostUpdated>,
    removed_chat_boost: Option<ChatBoostRemoved>,
}
