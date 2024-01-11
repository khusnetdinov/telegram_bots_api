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
    #[structopt(short, long, required_unless("help"))]
    pub token: String,

    /// Telegram api url
    #[structopt(short, long, default_value = "https://api.telegram.org/")]
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}
