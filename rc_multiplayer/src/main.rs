mod cli;
mod handler;
mod traits;
pub use traits::{EventCodeHandler, UserData, PacketData, EventCode, RobotMotionHandler, DisconnectHandler, Broadcastable};
mod data;
mod events;
mod handlers;
mod user;
mod matches;
mod vehicle_motion;
mod disconnect;

pub struct InitConfig {
    pub config: oj_rc_core::persist::config::ConfigImpl,
    pub users: std::sync::Arc<oj_rc_core::persist::user::UserImpl>,
    pub parsers: std::sync::Arc<oj_rc_core::cubes::CubeParsers>,
    pub matches_chann: tokio::sync::mpsc::Sender<matches::GameMessage>,
}

pub static START_TIMESTAMP_S: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = cli::CliArgs::get();
    log::debug!("Got cli args {:?}", args);

    let config = oj_rc_core::persist::config::ConfigImpl::load(&args.assets).expect("Bad config data");
    let users = std::sync::Arc::new(oj_rc_core::persist::user::UserImpl::load(&args.data, &config).await.expect("Bad user data"));
    users.multiplayer_init().await.expect("Multiplayer init task failed");
    let factory = std::sync::Arc::new(<oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::factory::<'_, '_>(&config).await.expect("Bad vehicle factory (CRF) config"));
    let parsers = std::sync::Arc::new(oj_rc_core::cubes::CubeParsers::new(&config));
    let matches = matches::GameMatches::new(&config, parsers.clone(), factory.clone());
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

    let start_time = chrono::Utc::now();
    START_TIMESTAMP_S.store(start_time.timestamp(), std::sync::atomic::Ordering::Relaxed);

    server.listen().await
}

pub async fn update_status(user_info: &dyn oj_rc_core::persist::user::IntercomUser, player_count: u64) {
    user_info.update_status(
        env!("CARGO_PKG_NAME"),
        oj_serdes::ServerStatus {
            uptime_s: (chrono::Utc::now().timestamp() - crate::START_TIMESTAMP_S.load(std::sync::atomic::Ordering::Relaxed)).try_into().unwrap_or_default(),
            players: player_count,
            version: env!("CARGO_PKG_VERSION").to_owned(),
        },
    ).await;
}
