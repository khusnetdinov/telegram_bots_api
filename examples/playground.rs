use std::error::Error;
use telegram::api::requests::sync::Requests;
use telegram::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let request_error = Client::new().sync.get_me().unwrap_err();
    println!("{:#?}", request_error);

    Ok(())
}
