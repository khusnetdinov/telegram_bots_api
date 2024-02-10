use std::error::Error;
use std::fs;
use telegram::api::responses::result::ResponseResult;
use telegram::api::types::update::Update;

fn main() -> Result<(), Box<dyn Error>> {
    let mock_response = fs::read_to_string("src/tests/responses/get_updates_success.json").unwrap();
    let mock_struct = serde_json::from_str::<ResponseResult<Vec<Update>>>(&mock_response).unwrap();

    println!("{:#?}", mock_struct);

    Ok(())
}
