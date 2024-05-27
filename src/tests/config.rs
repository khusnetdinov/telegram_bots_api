use crate::config::Config;

#[test]
fn new_with_default() {
    let config = Config::default();

    assert!(!config.debug);
    assert!(!config.production);

    assert_eq!(config.token, String::from(""));
    assert_eq!(config.url, String::from(""));
    assert_eq!(config.webhook, String::from(""));
    assert_eq!(config.timeout, 5u64);
    assert_eq!(config.connect_timeout, 5u64);
    assert_eq!(config.updates_offset, 0i64);
    assert_eq!(config.updates_limit, 100i64);
    assert_eq!(config.updates_timeout, 0u64);
}
