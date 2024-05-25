use std::error::Error;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_poll::SendPoll;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::clients::r#async::Async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let params = SendPoll {
        chat_id: ChatUId::from("@development_telegrapha_bot"),
        question: String::from("test poll"),
        ..SendPoll::default()
    };

    let response = Async::new().send_poll(&params).await?;

    // let params = StopPoll {
    //     message_id: MessageId::from(450),
    //     chat_id: ChatUId::from(-1001368460856),
    //     reply_markup: None
    // };
    //
    // let response = Async::new().stop_poll(&params).await?;

    println!("{:#?}", response);

    Ok(())
}
