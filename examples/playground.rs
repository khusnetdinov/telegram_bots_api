use std::error::Error;
use telegram::api::params::send_dice::SendDice;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "{}",
        SendDice {
            ..Default::default()
        }
        .into()
    );
    Ok(())
}
