use std::error::Error;
use telegram::api::requests::sync::Requests;
use telegram::client::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Client::new();

    // let params = telegram::api::params::UpdateParams { limit: 100, offset: 249563340, timeout: 0 };
    // let response = api.sync.get_updates(&params).unwrap();

    let response = api.sync.get_me().unwrap();

    println!("Response: {:#?}", response);

    Ok(())
}
