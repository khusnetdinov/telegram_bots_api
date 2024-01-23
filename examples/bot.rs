use std::error::Error;
use telegram::api::response::Success;
use telegram::api::types::User;
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

    let success: Success<User> = serde_json::from_str(&body).unwrap();

    println!("{:#?}", success);

    Ok(())
}
