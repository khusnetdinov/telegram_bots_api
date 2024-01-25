use std::error::Error;
use telegram::Api;

fn main() -> Result<(), Box<dyn Error>> {
    let api = Api::new();

    println!("{:#?}", api);

    Ok(())
}
