use std::error::Error;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::enums::file_input::FileInput;
use telegram_bots_api::api::params::send_photo::SendPhoto;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::clients::r#async::Async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let params = SendPhoto {
        chat_id: ChatUId::from(147951145),
        photo: FileInput::from("https://248006.selcdn.ru/main/iblock/73d/73da4a4a09e01c1a4b2f20d3a870ac62/f8c5806b72c401ebaa6a32a2a482a3d4.png".to_string()),
        ..Default::default()
    };

    match Async::from_env().send_photo(&params).await {
        Ok(response) => {
            println!("Photo was uploaded {response:?}");
        }
        Err(error) => {
            eprintln!("Failed to upload photo: {error:?}");
        }
    }
    Ok(())
}
