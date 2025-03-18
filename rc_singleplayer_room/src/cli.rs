use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// TCP port on which to accept connections
    #[arg(short, long, default_value_t = 4539)]
    pub port: u16,

    /// IP Address on which to accept connections
    #[arg(long, default_value_t = {"127.0.0.1".to_string()})]
    pub ip: String,

    /// Handle one connection and then exit
    #[arg(short = '1', long)]
    pub once: bool,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }
}
