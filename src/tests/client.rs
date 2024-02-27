use crate::clients::r#async::Async;
use crate::clients::sync::Sync;
use crate::config::Config;
use crate::Client;

#[test]
fn new() {
    let config = Config {
        ..Default::default()
    };
    let sync = Sync::new(&config);
    let r#async = Async::new(&config);

    let client = Client {
        config,
        sync,
        r#async,
    };

    assert_eq!(client.config.debug, false);
}
