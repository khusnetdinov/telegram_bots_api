use std::error::Error;
use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::clients::sync::Sync;

fn main() -> Result<(), Box<dyn Error>> {
    let response = Sync::from_env().get_me().unwrap();

    println!("{:#?}", response);

    Ok(())
}
