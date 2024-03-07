use std::error::Error;
use telegram::api::requests::r#async::Requests;
use telegram::clients::r#async::Async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = Async::new().get_me().await?;

    println!("{:#?}", response);

    Ok(())
}
