use std::error::Error;
use telegram::api::requests::blocking::Requests;
use telegram::client::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Client::new();

    let response = api.blocking.get_me().unwrap();

    println!("{:#?}", response);

    Ok(())
}
