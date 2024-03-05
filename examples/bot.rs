use std::error::Error;
use telegram::api::enums::chat_uid::ChatUId;
use telegram::api::params::send_message::SendMessage;
use telegram::api::requests::sync::Requests;
use telegram::api::types::chat_id::ChatId;
use telegram::clients::sync::Sync;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Sync::new();

    let params = SendMessage {
        chat_id: ChatUId::I64(ChatId(147951145)),
        text: "Hello World".to_string(),
        ..Default::default()
    };

    api.send_message(&params).unwrap();

    Ok(())
}
