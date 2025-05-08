use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[cfg(feature = "robocraft")]
    /// Robocraft user data root
    #[arg(long, default_value_t = {"../data/robocraft".to_string()})]
    pub data_robocraft: String,
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
            #[cfg(feature = "robocraft")]
            robocraft: crate::robocraft::RcConfig::from_args(&self).await,
        }
    }
}

pub struct Config {
    #[cfg(feature = "robocraft")]
    pub robocraft: crate::robocraft::RcConfig,
}
