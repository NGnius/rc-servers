mod cli;
mod state;

mod data;
mod operations;

use std::num::NonZero;
use std::sync::Arc;

use polariton_auth::Handshake;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

use polariton::packet::{Cryptographer, Data, Message, Packet, Ping, StandardMessage, StandardPacket};
use polariton::operation::{OperationResponse, Typed};

pub type UserTy = std::sync::RwLock<state::UserState>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = cli::CliArgs::get();
    log::debug!("Got cli args {:?}", args);

    let op_handler = Arc::new(operations::handler());

    let ip_addr: std::net::IpAddr = args.ip.parse().expect("Invalid IP address");

    let listener = net::TcpListener::bind(std::net::SocketAddr::new(ip_addr, args.port)).await?;

    #[cfg(not(debug_assertions))]
    loop {
        let (socket, address) = listener.accept().await?;
        tokio::spawn(process_socket(socket, address, NonZero::new(args.retries), op_handler.clone()));
    }
    #[cfg(debug_assertions)]
    {
        let (socket, address) = listener.accept().await?;
        process_socket(socket, address, NonZero::new(args.retries), op_handler.clone()).await;
        Ok(())
    }
}

async fn process_socket(mut socket: net::TcpStream, address: std::net::SocketAddr, retries: Option<NonZero<usize>>, op_handler: Arc<polariton_server::operations::OperationsHandler<crate::UserTy>>) {
    log::debug!("Accepting connection from address {}", address);

    let mut read_buf = Vec::new();
    let mut write_buf = Vec::new();
    let enc = match do_connect_handshake(&mut read_buf, &mut socket, retries).await {
        Some(x) => x,
        None => {
            log::error!("Failed to do connect handshake with {}", address);
            return;
        }
    };
    let sock_state = state::State::new(enc);
    let user_state = sock_state.user();
    while let Ok(packet) = receive_packet(&mut read_buf, &mut socket, retries, sock_state.binrw_args()).await {
        match packet {
            Packet::Ping(ping) => {
                handle_ping(ping, &mut write_buf, &mut socket).await;
                for _ in 0..5 {
                    read_buf.remove(0);
                }
            },
            Packet::Packet(packet) => {
                // remove packet's advertised size from the buffer
                for _ in 0..packet.header.len {
                    read_buf.remove(0);
                }
                match packet.message {
                    Message::Ping(ping) => {
                        handle_ping(ping, &mut write_buf, &mut socket).await;
                    },
                    Message::Standard(msg) => {

                        let is_encrypted = msg.is_encrypted();
                        match msg.data {
                            Data::OpReq(req) => {
                                let resp = op_handler.handle_op(&user_state, req);
                                let result = send_packet(
                                    Packet::from_message(
                                        Message::Standard(StandardMessage {
                                            flags: 0,
                                            data: Data::OpResp(resp),
                                        }.encrypt(is_encrypted)),
                                        packet.header.channel,
                                        packet.header.is_reliable(),
                                        sock_state.binrw_args()).unwrap(),
                                    &mut write_buf, &mut socket, sock_state.binrw_args()).await;
                                match result {
                                    Ok(_) => {},
                                    Err(e) => {
                                        log::error!("Failed to send operation response packet: {}", e);
                                    }
                                }
                            },
                            data => log::warn!("Failed to handle packet with message data {:?}", data),
                        }
                    }
                }
            }
            //log::warn!("Not handling packet {:?}", packet),
        }
    }
    log::debug!("Goodbye connection from address {}", address);
}

async fn handle_ping(ping: Ping, buf: &mut Vec<u8>, socket: &mut net::TcpStream) {
    buf.clear();
    let resp = Packet::Ping(polariton_auth::ping_pong(ping));
    resp.to_buf(buf, None).unwrap();
    let write_count = socket.write(buf).await.unwrap();
    log::debug!("(ping) Write {} bytes to socket: {:?}", write_count, buf);
    buf.clear();
}

fn buf_likely_valid(buf: &[u8]) -> bool {
    buf.is_empty() || buf[0] == Packet::PING_MAGIC || buf[0] == Packet::FRAMED_MAGIC
}

async fn read_more(buf: &mut Vec<u8>, socket: &mut net::TcpStream) -> Result<usize, std::io::Error> {
    let read_count = socket.read_buf(buf).await?;
    log::debug!("Read {} bytes from socket: {:?}", read_count, buf);
    Ok(read_count)
}

async fn receive_packet(buf: &mut Vec<u8>, socket: &mut net::TcpStream, max_retries: Option<NonZero<usize>>, args: Option<Box<Arc<dyn Cryptographer + 'static>>>) -> Result<Packet, std::io::Error> {
    if buf.is_empty() {
        let read_count = read_more(buf, socket).await?;
        if read_count == 0 { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "socket did not read any bytes")); } // bad packet
    }

    let mut last_err = None;
    let mut must_succeed_next = false;
    if let Some(max_retries) = max_retries {
        for _ in 0..max_retries.get() {
            match Packet::from_buf(&buf, args.clone()) {
                Ok(packet) => {
                    log::debug!("Received packet {:?}", packet);
                    return Ok(packet);
                },
                Err(e) => last_err = Some(e),
            }
            if must_succeed_next {
                break;
            }
            must_succeed_next = read_more(buf, socket).await? == 0;
        }
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, last_err.unwrap()));
    } else {
        while buf_likely_valid(buf.as_slice()) {
            match Packet::from_buf(&buf, args.clone()) {
                Ok(packet) => {
                    log::debug!("Received packet {:?}", packet);
                    return Ok(packet);
                },
                Err(e) => last_err = Some(e),
            }
            if must_succeed_next {
                break;
            }
            must_succeed_next = read_more(buf, socket).await? == 0;
        }
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, last_err.unwrap()));
    }
}

async fn send_packet(packet: Packet, buf: &mut Vec<u8>, socket: &mut net::TcpStream, args: Option<Box<Arc<dyn Cryptographer>>>) -> Result<(), std::io::Error> {
    log::debug!("Sending packet {:?}", packet);
    buf.clear();
    packet.to_buf(buf, args).map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let write_count = socket.write(buf).await?;
    log::debug!("Write {} bytes to socket: {:?}", write_count, buf);
    #[cfg(debug_assertions)]
    {
        // print out unencrypted packet too
        if let Packet::Packet(standard_p) = packet {
            if let Message::Standard(standard_m) = standard_p.message {
                if standard_m.is_encrypted() {
                    let standard_m = standard_m.encrypt(false);
                    let packet = Packet::Packet(StandardPacket { header: standard_p.header, message: Message::Standard(standard_m) });
                    packet.to_buf(buf, None).map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
                    log::debug!("Unencrypted bytes of packet: {:?} (len: {})", buf, buf.len());
                }

            }
        }
    }
    buf.clear();
    Ok(())
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
    buf: &mut Vec<u8>,
    socket: &mut net::TcpStream,
    max_retries: Option<NonZero<usize>>,
) -> Option<Box<std::sync::Arc<dyn Cryptographer>>> {
    let handshake = Handshake::new(APP_ID);
    // connect
    log::debug!("(connect) Handling first packet");
    let packet1 = match receive_packet(buf, socket, max_retries, None).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read connect packet: {}", e);
            return None;
        }
    };
    buf.clear();
    let (handshake, to_send) = match handshake.connect(&packet1) {
        Ok(x) => (x.handshake, x.extra),
        Err(e) => {
            log::error!("Failed to handle connect handshake: {:?}", e.extra);
            return None;
        }
    };
    match send_packet(to_send, buf, socket, None).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Failed to send connect ack packet: {}", e);
            return None;
        }
    }
    // encrypt
    log::debug!("(connect) Handling second packet");
    let mut packet2 = match receive_packet(buf, socket, max_retries, None).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read (maybe) public key packet: {}", e);
            return None;
        }
    };
    buf.clear();
    while let Packet::Ping(ping) = packet2 {
        handle_ping(ping, buf, socket).await;
        packet2 = match receive_packet(buf, socket, max_retries, None).await {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to read (maybe) public key packet: {}", e);
                return None;
            }
        };
        buf.clear();
    }
    let (handshake, to_send, crypto) = match handshake.encrypt(&packet2) {
        Ok(x) => (x.handshake, x.extra.0, x.extra.1),
        Err(e) => {
            log::error!("Failed to handle encryption handshake: {:?}", e.extra);
            return None;
        }
    };
    match send_packet(to_send, buf, socket, None).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Failed to send encryption ack packet: {}", e);
            return None;
        }
    }
    // pre-auth
    let handshake = handshake.with_auth(AuthImpl);
    // authenticate
    log::debug!("(connect) Handling third packet");
    let mut packet3 = match receive_packet(buf, socket, max_retries, Some(crypto.clone())).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read (maybe) auth packet: {}", e);
            return None;
        }
    };
    buf.clear();
    while let Packet::Ping(ping) = packet3 {
        handle_ping(ping, buf, socket).await;
        packet3 = match receive_packet(buf, socket, max_retries, Some(crypto.clone())).await {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to read (maybe) auth packet: {}", e);
                return None;
            }
        };
        buf.clear();
    }
    let to_send = match handshake.authenticate(&packet3, crypto.clone()) {
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
    match send_packet(to_send, buf, socket, Some(crypto.clone())).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Failed to send auth ack packet: {}", e);
            return None;
        }
    }

    // join lobby
    log::debug!("(join lobby) Handling fourth packet");
    let mut packet_j = match receive_packet(buf, socket, max_retries, Some(crypto.clone())).await {
        Ok(x) => x,
        Err(e) => {
            log::error!("Failed to read (maybe) join packet: {}", e);
            return None;
        }
    };
    buf.clear();
    while let Packet::Ping(ping) = packet_j {
        handle_ping(ping, buf, socket).await;
        packet_j = match receive_packet(buf, socket, max_retries, Some(crypto.clone())).await {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to read (maybe) join packet: {}", e);
                return None;
            }
        };
        buf.clear();
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
                            }.encrypt(true)), 0, true, Some(crypto.clone())).unwrap();
                    match send_packet(resp, buf, socket, Some(crypto.clone())).await {
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
    buf.clear();

    Some(crypto)
}
