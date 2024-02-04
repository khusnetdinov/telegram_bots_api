use structopt::StructOpt;

#[derive(StructOpt, Debug, Default, PartialEq)]
#[structopt(name = "base")]
pub struct Config {
    /// Environment: Debug mode.
    #[structopt(short, long)]
    pub debug: bool,

    /// Environment: Is production.
    #[structopt(short, long)]
    pub production: bool,

    /// Telegram: Token.
    #[structopt(long, required_unless("help"))]
    pub token: String,

    /// Telegram: Api url.
    #[structopt(long, default_value = "https://api.telegram.org/")]
    pub url: String,

    /// Client: Timeout in secs. The timeout is applied from when the request starts connecting until the response body has finished.
    #[structopt(long, default_value = "5")]
    pub timeout: u64,

    /// Client: Connect timeout in secs. Set a timeout for only the connect phase.
    #[structopt(long, default_value = "5")]
    pub connect_timeout: u64,

    /// Updates: Identifier of the first update to be returned.
    #[structopt(long, default_value = "0")]
    pub updates_offset: i64,

    /// Updates: Limits the number of updates to be retrieved.
    #[structopt(long, default_value = "100")]
    pub updates_limit: i64,

    /// Updates: Timeout in seconds for long polling.
    #[structopt(long, default_value = "0")]
    pub updates_timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}

impl Config {
    pub fn build_url(&self) -> String {
        format!("{}bot{}/", self.url, self.token)
    }
}
