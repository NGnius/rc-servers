use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// TCP port on which to accept connections
    #[arg(short, long, default_value_t = 4532)]
    pub port: u16,

    /// IP Address on which to accept connections
    #[arg(long, default_value_t = {"127.0.0.1".to_string()})]
    pub ip: String,

    /// Socket read tries before giving up (0 to never give up)
    #[arg(long, default_value_t = 5)]
    pub retries: usize,

    /// Domain and port of the game server to send new connections
    #[arg(long, default_value_t = {"127.0.0.1:4533".to_string()})]
    pub redirect: String,

    /// Name of game server to send new connections
    #[arg(long, default_value_t = {"ngram_is_ngnius".to_string()})]
    pub room_name: String,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }
}
