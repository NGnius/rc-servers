use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// TCP port on which to accept connections
    #[arg(short, long, default_value_t = 8002)]
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
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }

    pub fn loaded(&self) -> LoadedArgs {
        let assets_path = std::path::PathBuf::from(&self.assets_robocraft);
        let data_path = std::path::PathBuf::from(&self.data_robocraft);
        let token_path = data_path.join(oj_rc_core::persist::user::TOKEN_SECRET_FILENAME);
        let secret = std::fs::read(&token_path).expect("Bad token");
        let cookie_key = if secret.len() < 32 {
            log::warn!("{} should be >= 32 bytes (extending with zeroes)", token_path.display());
            let mut secret_ext = secret.clone();
            while secret_ext.len() < 32 {
                secret_ext.push(0);
            }
            actix_web::cookie::Key::derive_from(&secret_ext)
        } else {
            actix_web::cookie::Key::derive_from(&secret)
        };
        LoadedArgs {
            secret: std::sync::Arc::new(secret.clone()),
            cookie_key,
            assets: assets_path,
            data: data_path,
        }
    }
}

#[allow(dead_code)]
pub struct LoadedArgs {
    pub secret: std::sync::Arc<Vec<u8>>,
    pub cookie_key: actix_web::cookie::Key,
    pub assets: std::path::PathBuf,
    pub data: std::path::PathBuf,
}
