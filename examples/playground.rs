use std::error::Error;
use telegram::api::enums::chat_uid::ChatUId;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", ChatUId::from("khusnetdinov"));

    Ok(())
}
