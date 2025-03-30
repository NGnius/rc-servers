mod cli;

mod data;
mod events;
mod operations;

use polariton_auth::Handshake;
use tokio::net;

use polariton::packet::{Data, Message, Packet, StandardMessage};
use polariton::operation::{OperationResponse, Typed};

pub type UserTy = rc_core::UserState<()>;

pub struct InitConfig {
    pub cubes: rc_core::persist::config::ConfigImpl,
    pub users: std::sync::Arc<rc_core::persist::user::UserImpl>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = cli::CliArgs::get();
    log::debug!("Got cli args {:?}", args);

    let cubes = rc_core::persist::config::ConfigImpl::load(&args.assets).expect("Bad config data");
    let users = std::sync::Arc::new(rc_core::persist::user::UserImpl::load(&args.data, &cubes).expect("Bad user data"));
    let init_ctx = std::sync::Arc::new(InitConfig {
        cubes,
        users,
    });

    let server = std::sync::Arc::new(polariton_server::Server::new(operations::handler(&init_ctx), polariton_server::events::EventsHandler::new()));

    let ip_addr: std::net::IpAddr = args.ip.parse().expect("Invalid IP address");

    let listener = net::TcpListener::bind(std::net::SocketAddr::new(ip_addr, args.port)).await?;

    if args.once {
        log::warn!("Handling first connection and then exiting");
        let (socket, address) = listener.accept().await?;
        process_socket(socket, address, server.clone(), init_ctx).await;
    } else {
        loop {
            let (socket, address) = listener.accept().await?;
            tokio::spawn(process_socket(socket, address, server.clone(), init_ctx.clone()));
        }
    }
    server.join();
    server.join_async().await;
    Ok(())
}

async fn process_socket(mut socket: net::TcpStream, address: std::net::SocketAddr, server: std::sync::Arc<polariton_server::Server<crate::UserTy>>, init_ctx: std::sync::Arc<InitConfig>) {
    log::debug!("Accepting connection from address {}", address);
    let enc = match do_connect_handshake(&mut socket).await {
        Some(x) => x,
        None => {
            log::error!("Failed to do connect handshake with {}", address);
            return;
        }
    };
    let (socket_r, socket_w) = socket.into_split();
    let (chann_tx, chann_rx) = tokio::sync::mpsc::unbounded_channel();
    let user_state = rc_core::UserState::<()>::new(init_ctx.users.clone(), chann_tx.clone());
    let ctx = polariton::packet::SerdesContext::from_boxed(Default::default(), enc);
    server.handle_async_with_channel(socket_r, socket_w, user_state, ctx, chann_tx, chann_rx).await;
    log::debug!("Goodbye connection from address {}", address);
}

const APP_ID: &str = "WebServicesServer";

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

impl polariton_auth::AuthProvider<AuthError> for AuthImpl {
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
            polariton_auth::AuthError::Validation(e) => {
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
