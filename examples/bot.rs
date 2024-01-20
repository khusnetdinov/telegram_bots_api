use std::error::Error;
use telegram::Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api = Api::new();

    let res = api
        .client
        .post(format!("{}{}", api.url, "getMe"))
        .send()
        .await?;
    let body = res.text().await?;

    println!("{:#?}", body);

    Ok(())
}
