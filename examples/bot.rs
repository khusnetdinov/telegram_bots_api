use std::error::Error;
use telegram::api::requests::Requests;
use telegram::Api;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Api::new();

    let response = api.blocking.get_me().unwrap();

    println!("{:#?}", response);

    Ok(())
}
