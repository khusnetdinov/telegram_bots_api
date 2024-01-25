use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "base")]
pub struct Config {
    /// Debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Production environment
    #[structopt(short, long)]
    pub production: bool,

    /// Telegram token
    #[structopt(long, required_unless("help"))]
    pub token: String,

    /// Telegram api url
    #[structopt(long, default_value = "https://api.telegram.org/")]
    pub url: String,

    /// Http timeout in secs. The timeout is applied from when the request starts connecting until the response body has finished.
    #[structopt(long, default_value = "5")]
    pub timeout: u64,

    /// Http connect timeout in secs. Set a timeout for only the connect phase.
    #[structopt(long, default_value = "5")]
    pub connect_timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}

pub trait Builder {
    fn build_client(&self) -> reqwest::blocking::Client;
    fn build_url(&self) -> String;
}

impl Builder for Config {
    fn build_client(&self) -> reqwest::blocking::Client {
        reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(self.timeout))
            .connect_timeout(Duration::from_secs(self.connect_timeout))
            .build()
            .unwrap()
    }

    fn build_url(&self) -> String {
        format!("{}bot{}/", self.url, self.token)
    }
}
