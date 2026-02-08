#![forbid(unsafe_code)]
mod cli;
mod state;
mod op_handler;
pub use op_handler::SimpleChatFunc;
mod plugin_wrapper;
pub use plugin_wrapper::{PluginWrapper, ProviderWrapper};

mod data;
mod operations;
mod events;

use oj_polariton_auth::Handshake;
use oj_rc_core::ConfigProvider;
use tokio::net;

use polariton::packet::{Data, Message, Packet, StandardMessage};
use polariton::operation::{OperationResponse, Typed};

pub type UserTy = std::sync::Arc<oj_rc_core::UserState>;

pub static START_TIMESTAMP_S: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
pub static READY_DURATION_NS: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
pub static ONLINE_USERS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LOGINS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let start_time = chrono::Utc::now();
    START_TIMESTAMP_S.store(start_time.timestamp(), std::sync::atomic::Ordering::Relaxed);
    env_logger::init();
    let args = cli::CliArgs::get();
    log::debug!("Got cli args {:?}", args);

    let cubes = oj_rc_core::persist::config::ConfigImpl::load(&args.assets).expect("Bad config data");
    let users = std::sync::Arc::new(oj_rc_core::persist::user::UserImpl::load(&args.data, &cubes).await.expect("Bad user data"));

    let chat_plugin_path = std::path::PathBuf::from(&args.data).join("plugins/chat");
    let chat_system = state::chat::ChatImpl::new(<oj_rc_core::ConfigImpl as ConfigProvider<()>>::chat_system_config(&cubes), chat_plugin_path).expect("Bad chat config data");

    let server = std::sync::Arc::new(polariton_server::Server::new(operations::handler(chat_system.clone(), &cubes), polariton_server::events::EventsHandler::new()));

    let ip_addr: std::net::IpAddr = args.ip.parse().expect("Invalid IP address");

    let listener = net::TcpListener::bind(std::net::SocketAddr::new(ip_addr, args.port)).await?;

    let ready_dur = chrono::Utc::now() - start_time;
    READY_DURATION_NS.store(ready_dur.num_nanoseconds().unwrap_or(-1), std::sync::atomic::Ordering::Relaxed);
    log::info!("chat_room ready");
    if args.once {
        log::warn!("Handling first connection and then exiting");
        let (socket, address) = listener.accept().await?;
        process_socket(socket, address, server.clone(), users.clone(), chat_system).await;
    } else {
        loop {
            let (socket, address) = listener.accept().await?;
            tokio::spawn(process_socket(socket, address, server.clone(), users.clone(), chat_system.clone()));
        }
    }
    server.join();
    server.join_async().await;
    Ok(())
}

async fn process_socket(mut socket: net::TcpStream, address: std::net::SocketAddr, server: std::sync::Arc<polariton_server::Server<crate::UserTy>>, users: std::sync::Arc<oj_rc_core::persist::user::UserImpl>, chat: state::chat::ChatImpl) {
    log::debug!("Accepting connection from address {}", address);
    let enc = match do_connect_handshake(&mut socket).await {
        Some(x) => x,
        None => {
            log::error!("Failed to do connect handshake with {}", address);
            return;
        }
    };
    LOGINS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    ONLINE_USERS.store(chat.system().await.user_count() as u64 + 1, std::sync::atomic::Ordering::SeqCst);
    let (chann_tx, chann_rx) = tokio::sync::mpsc::unbounded_channel();
    let user_state = std::sync::Arc::new(oj_rc_core::UserState::<()>::new(users, chann_tx.clone()));
    let (socket_r, socket_w) = socket.into_split();
    server.handle_async_with_channel_join(socket_r, socket_w, user_state.clone(), polariton::packet::SerdesContext::from_boxed(Default::default(), enc), chann_tx, chann_rx).await;
    log::debug!("Goodbye connection from address {}", address);
    ONLINE_USERS.store(chat.system().await.user_count() as u64 - 1, std::sync::atomic::Ordering::SeqCst);
    if let Ok(user_info) = user_state.user() {
        update_status(user_info.as_ref().as_ref()).await;
    }
}

const APP_ID: &str = "ChatServer";

struct AuthImpl;

const TOKEN_KEY: u8 = 216; // token;refresh_token
//const UNKNOWN_BYTE_KEY: u8 = 217;
const SERVICE_KEY: u8 = 224;
const USERNAME_KEY: u8 = 225;

//const CCU_KEY: u8 = 245;

#[derive(Debug)]
enum AuthError {
    WrongService { expected: String, actual: String },
    MissingService,
    MissingToken,
    MissingUsername,
}

impl AuthError {
    fn log_err(&self) {
        match self {
            Self::WrongService { expected, actual } => log::error!("(auth fail) Got unexpected service {}, expected {}", actual, expected),
            Self::MissingService => log::error!("(auth fail) No service name param ({}) received", SERVICE_KEY),
            Self::MissingToken => log::error!("(auth fail) No token param ({}) received", TOKEN_KEY),
            Self::MissingUsername => log::error!("(auth fail) No username param ({}) received", USERNAME_KEY),
        }
    }
}

impl oj_polariton_auth::AuthProvider<AuthError> for AuthImpl {
    fn validate(&mut self, params: &std::collections::HashMap<u8, Typed>) -> Result<std::collections::HashMap<u8, Typed>, AuthError> {
        if let Some(Typed::Str(token)) = params.get(&TOKEN_KEY) {
            if let Some(Typed::Str(service)) = params.get(&SERVICE_KEY) {
                if let Some(Typed::Str(user)) = params.get(&USERNAME_KEY) {
                    if service.string == APP_ID {
                        let params_resp = std::collections::HashMap::<u8, Typed>::new();
                        //params_resp.insert(CCU_KEY, Typed::Byte(0));
                        log::debug!("Auth success for {} (token: {})", user.string, token.string);
                        Ok(params_resp)
                    } else { Err(AuthError::WrongService { expected: APP_ID.to_owned(), actual: service.string.to_owned() }) }
                } else { Err(AuthError::MissingUsername) }
            } else { Err(AuthError::MissingService) }
        } else { Err(AuthError::MissingToken) }
    }
}

async fn do_connect_handshake(
    socket: &mut net::TcpStream,
) -> Option<Box<dyn polariton::packet::Cryptographer>> {
    let handshake = Handshake::new(APP_ID);
    // connect
    log::debug!("(connect) Handling first packet");
    let packet1 = match polariton_server::utils::receive_packet_async(socket, &Default::default()).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read connect packet: {}", e);
            return None;
        }
    };
    let (handshake, to_send) = match handshake.connect(&packet1) {
        Ok(x) => (x.handshake, x.extra),
        Err(e) => {
            log::error!("Failed to handle connect handshake: {:?}", e.extra);
            return None;
        }
    };
    match polariton_server::utils::send_packet_async(&to_send, socket, &Default::default()).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Failed to send connect ack packet: {}", e);
            return None;
        }
    }
    // encrypt
    log::debug!("(connect) Handling second packet");
    let mut packet2 = match polariton_server::utils::receive_packet_async(socket, &Default::default()).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read (maybe) public key packet: {}", e);
            return None;
        }
    };
    while let Packet::Ping(ping) = packet2 {
        polariton_server::utils::handle_ping_async(ping, socket, &Default::default()).await.unwrap_or_default();
        packet2 = match polariton_server::utils::receive_packet_async(socket, &Default::default()).await {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to read (maybe) public key packet: {}", e);
                return None;
            }
        };
    }
    let (handshake, to_send, crypto) = match handshake.encrypt(&packet2) {
        Ok(x) => (x.handshake, x.extra.0, x.extra.1),
        Err(e) => {
            log::error!("Failed to handle encryption handshake: {:?}", e.extra);
            return None;
        }
    };
    match polariton_server::utils::send_packet_async(&to_send, socket, &Default::default()).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Failed to send encryption ack packet: {}", e);
            return None;
        }
    }
    // pre-auth
    let handshake = handshake.with_auth(AuthImpl);
    let op_ctx = polariton::serdes::SerdesContext::default();
    let ctx = polariton::packet::SerdesContext::new(op_ctx, crypto);
    // authenticate
    log::debug!("(connect) Handling third packet");
    let mut packet3 = match polariton_server::utils::receive_packet_async(socket, &ctx).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read (maybe) auth packet: {}", e);
            return None;
        }
    };
    while let Packet::Ping(ping) = packet3 {
        polariton_server::utils::handle_ping_async(ping, socket, &Default::default()).await.unwrap_or_default();
        packet3 = match polariton_server::utils::receive_packet_async(socket, &ctx).await {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to read (maybe) auth packet: {}", e);
                return None;
            }
        };
    }
    let to_send = match handshake.authenticate(&packet3, &ctx) {
        Ok(x) => x,
        Err(h) => match h.extra {
            oj_polariton_auth::AuthError::Validation(e) => {
                e.log_err();
                return None;
            },
            e => {
                log::error!("Failed to handle auth handshake: {:?}", e);
                return None;
            },
        },
    };
    match polariton_server::utils::send_packet_async(&to_send, socket, &ctx).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Failed to send auth ack packet: {}", e);
            return None;
        }
    }

    // join lobby
    log::debug!("(join lobby) Handling fourth packet");
    let mut packet_j = match polariton_server::utils::receive_packet_async(socket, &ctx).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read (maybe) join packet: {}", e);
            return None;
        }
    };
    while let Packet::Ping(ping) = packet_j {
        polariton_server::utils::handle_ping_async(ping, socket, &Default::default()).await.unwrap_or_default();
        packet_j = match polariton_server::utils::receive_packet_async(socket, &ctx).await {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to read (maybe) join packet: {}", e);
                return None;
            }
        };
    }
    if let Packet::Packet(msg) = &packet_j {
        if let Message::Standard(st) = &msg.message {
            if let Data::OpReq(req) = &st.data {
                if req.code == 226 { // join lobby (but for real this time)
                    let mut params = std::collections::HashMap::<u8, Typed>::new();
                    //params.insert(252 /* actors in game */, Typed::Str(game_server_url.into()));
                    params.insert(254 /* game server address */, Typed::Int(42));
                    params.insert(249 /* actor properties */, Typed::HashMap(Vec::new().into()));
                    params.insert(248 /* game properties */, Typed::HashMap(Vec::new().into()));
                    let resp = Packet::from_message(
                        Message::Standard(
                            StandardMessage { flags: 0,
                                data: Data::OpResp(OperationResponse {
                                    code: req.code,
                                    return_code: 0,
                                    message: Typed::Null,
                                    params: params.into(),
                                }),
                            }.encrypt(true)), 0, true, &ctx).unwrap();
                    match polariton_server::utils::send_packet_async(&resp, socket, &ctx).await {
                        Ok(_) => {},
                        Err(e) => {
                            log::error!("Failed to send lobby ack packet: {}", e);
                            return None;
                        }
                    }
                }
            }
        }
    }

    Some(ctx.into_crypto())
}

pub async fn update_status(user_info: &dyn oj_rc_core::persist::user::IntercomUser) {
    let version = env!("CARGO_PKG_VERSION");
    let git_version = git_version::git_version!(args = ["--always", "--dirty=+"]);
    let full_version = format!("{}:{}", version, git_version);
    user_info.update_status(
        env!("CARGO_PKG_NAME"),
        oj_serdes::ServerStatus {
            uptime_s: (chrono::Utc::now().timestamp() - crate::START_TIMESTAMP_S.load(std::sync::atomic::Ordering::Relaxed)).try_into().unwrap_or_default(),
            players: ONLINE_USERS.load(std::sync::atomic::Ordering::SeqCst),
            version: full_version,
        },
    ).await;
}
