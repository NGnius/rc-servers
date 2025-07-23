mod cli;
mod handler;
mod traits;
pub use traits::{EventCodeHandler, UserData, PacketData, EventCode, RobotMotionHandler, Broadcastable};
mod data;
mod events;
mod handlers;
mod user;
mod matches;
mod vehicle_motion;

pub struct InitConfig {
    pub config: oj_rc_core::persist::config::ConfigImpl,
    pub users: std::sync::Arc<oj_rc_core::persist::user::UserImpl>,
    pub parsers: oj_rc_core::cubes::CubeParsers,
    pub matches_chann: tokio::sync::mpsc::Sender<matches::GameMessage>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = cli::CliArgs::get();
    log::debug!("Got cli args {:?}", args);

    let config = oj_rc_core::persist::config::ConfigImpl::load(&args.assets).expect("Bad config data");
    let users = std::sync::Arc::new(oj_rc_core::persist::user::UserImpl::load(&args.data, &config).await.expect("Bad user data"));
    users.multiplayer_init().await.expect("Multiplayer init task failed");
    let parsers = oj_rc_core::cubes::CubeParsers::new(&config);
    let matches = matches::GameMatches::new(&config);
    let matches_chann = matches.spawn();

    let init_ctx = InitConfig {
        config,
        users,
        parsers,
        matches_chann,
    };

    let mtu = oj_rc_core::ConfigProvider::<()>::network_config(&init_ctx.config).max_packet_size;
    let event_handler = events::handler(&init_ctx).await;
    let server = literustlib_server::Server::new(event_handler, (args.ip, args.port), mtu).await.expect("Bad server");

    server.listen().await
}
