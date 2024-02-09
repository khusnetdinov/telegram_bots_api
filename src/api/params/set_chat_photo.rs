use crate::api::types::input_file::InputFile;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatphoto
/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatPhoto {
    chat_id: i64,
    photo: InputFile,
}
