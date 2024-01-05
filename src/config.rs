use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "base")]
pub struct Config {
    /// Debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}
