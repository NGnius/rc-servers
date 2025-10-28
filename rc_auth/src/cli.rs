use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// TCP port on which to accept connections
    #[arg(short, long, default_value_t = 8001)]
    pub port: u16,

    /// IP Address on which to accept connections
    #[arg(long, default_value_t = {"127.0.0.1".to_string()})]
    pub ip: String,

    /// Assets root
    #[arg(long, default_value_t = {"../assets/robocraft".to_string()})]
    pub assets_robocraft: String,

    /// User data root
    #[arg(long, default_value_t = {"../data/robocraft".to_string()})]
    pub data_robocraft: String,

    /// Verify configuration and then exit
    #[arg(long)]
    pub validate: bool,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }

    pub async fn preloaded(self) -> crate::robocraft::RcConfig {
        crate::robocraft::RcConfig::from_args(&self).await
    }
}
