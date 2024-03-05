use std::error::Error;
use telegram::api::requests::sync::Requests;
use telegram::clients::sync::Sync;

fn main() -> Result<(), Box<dyn Error>> {
    let response = Sync::new().get_me().unwrap();

    println!("{:#?}", response);

    Ok(())
}
