use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Robocraft asset data root
    #[arg(long, default_value_t = {"../assets/robocraft".to_string()})]
    pub assets_robocraft: String,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }

    pub async fn preloaded(self) -> Config {
        Config {
        }
    }
}

pub struct Config {
}
