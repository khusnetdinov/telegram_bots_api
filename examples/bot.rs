use telegram::Api;

fn main() {
    let api = Api::new().unwrap();

    println!("{:?}", api);
}
