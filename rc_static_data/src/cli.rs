use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// URL of authentication web server
    #[arg(long, default_value_t = {"http://127.0.0.1:8001/".to_string()})]
    pub auth: String,
    /// URL of microtransactions web server
    #[arg(long, default_value_t = {"http://127.0.0.1:8011/".to_string()})]
    pub pay: String,
}

impl CliArgs {
    pub fn get() -> Self {
        Self::parse()
    }
}
