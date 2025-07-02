mod cli;
mod handler;
mod traits;
pub use traits::{EventCodeHandler, UserData, PacketData};
mod data;

pub struct InitConfig {
    pub config: oj_rc_core::persist::config::ConfigImpl,
    pub users: std::sync::Arc<oj_rc_core::persist::user::UserImpl>,
    pub parsers: oj_rc_core::cubes::CubeParsers,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = cli::CliArgs::get();
    log::debug!("Got cli args {:?}", args);

    let config = oj_rc_core::persist::config::ConfigImpl::load(&args.assets).expect("Bad config data");
    let users = std::sync::Arc::new(oj_rc_core::persist::user::UserImpl::load(&args.data, &config).await.expect("Bad user data"));
    let parsers = oj_rc_core::cubes::CubeParsers::new(&config);

    let init_ctx = InitConfig {
        config,
        users,
        parsers,
    };

    let event_handler = handler::LnlEventHandler::new(&init_ctx).await;
    let server = literustlib_server::Server::new(event_handler, (args.ip, args.port), 4223).await.expect("Bad server");
    server.listen().await
}
