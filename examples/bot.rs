use std::error::Error;
use telegram::api::response::{ResponseError, ResponseSuccess};
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
    println!("JSON: {:#?}", body);

    let success: Result<ResponseSuccess<User>, serde_json::Error> = serde_json::from_str(&body);
    let error: Result<ResponseError, serde_json::Error> = serde_json::from_str(&body);

    println!("Success: {:#?}", success);
    println!("Error: {:#?}", error);

    Ok(())
}
