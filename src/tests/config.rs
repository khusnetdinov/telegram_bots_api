#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn build_url() {
        let token = String::from("token");
        let url = String::from("http://localhost");
        let config = Config {
            token: token.clone(),
            url: url.clone(),
            ..Default::default()
        };

        assert_eq!(config.build_url(), format!("{}bot{}/", url, token));
    }
}
