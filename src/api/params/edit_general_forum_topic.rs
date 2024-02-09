use serde::Serialize;

/// https://core.telegram.org/bots/api#editgeneralforumtopic
/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct EditGeneralForumTopic {
    chat_id: i64,
    name: String,
}
