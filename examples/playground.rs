use std::error::Error;
use std::fs;
use telegram::api::enums::menu_button::MenuButton;
use telegram::api::responses::result::ResponseResult;

fn main() -> Result<(), Box<dyn Error>> {
    let response_string =
        fs::read_to_string("src/tests/responses/get_chat_menu_button.json").unwrap();

    let response = serde_json::from_str::<ResponseResult<MenuButton>>(&response_string).unwrap();

    println!("{:#?}", response);

    Ok(())
}
