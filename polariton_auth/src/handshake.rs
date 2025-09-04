use polariton::packet::{Packet, Message, StandardMessage, Data};
use polariton::operation::{Typed, ParameterTable, OperationResponse};

#[derive(Debug)]
pub struct Handshake<T> {
    state: T,
}

pub struct HandshakeAnd<T, X> {
    pub handshake: Handshake<T>,
    pub extra: X,
}

pub struct Start<'a> {
    app_id: &'a str,
}

#[derive(Debug)]
pub enum ConnectError<'b, 'a> {
    UnexpectedPacket,
    WrongAppId { got: &'b str, expected: &'a str },
}

// TODO impl core::fmt::Display for ConnectError
// TODO impl std::error::Error for ConnectError

impl <'a> Handshake<Start<'a>> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            state: Start { app_id },
        }
    }

    pub fn connect<'b>(self, packet: &'b Packet) -> Result<HandshakeAnd<Connected, Packet>, HandshakeAnd<Start<'a>, ConnectError<'b, 'a>>> {
        if let Packet::Packet(packet) = &packet {
            if let Message::Standard(conn) = &packet.message {
                if let Data::InitStart(info) = &conn.data {
                    if info.app_id != self.state.app_id {
                        let err = ConnectError::WrongAppId { got: &info.app_id, expected: self.state.app_id };
                        return Err(HandshakeAnd {
                            handshake: self,
                            extra: err,
                        });
                    }
                    let new_self = Handshake::<Connected> {
                        state: Connected,
                    };
                    return Ok(HandshakeAnd {
                        handshake: new_self,
                        extra: Packet::from_message(
                            Message::Standard(StandardMessage {
                                flags: 0,
                                data: Data::InitAck ,
                            }),
                            packet.header.channel, true, &Default::default()).unwrap()
                    });
                }
            }
        }
        Err(HandshakeAnd {
            handshake: self,
            extra: ConnectError::UnexpectedPacket,
        })
    }
}

pub struct Connected;

#[derive(Debug)]
pub enum EncryptError {
    UnexpectedPacket,
    MissingParameter(u8),
}

impl Handshake<Connected> {
    const PUBLIC_KEY_PARAM_KEY: u8 = 1;

    pub fn encrypt(self, packet: &Packet) -> Result<HandshakeAnd<Encrypted, (Packet, crate::encryption::CryptoImpl)>, HandshakeAnd<Connected, EncryptError>> {
        if let Packet::Packet(packet) = &packet {
            if let Message::Standard(conn) = &packet.message {
                if let Data::InternalOpReq(req) = &conn.data {
                    if req.code == 0 {
                        let params = req.params.to_owned().to_dict();
                        if let Some(Typed::Bytes(pub_key)) = params.get(&Self::PUBLIC_KEY_PARAM_KEY) {
                            let keys = crate::encryption::generate_encryption_details(&pub_key.vec);
                            let mut response_params = std::collections::HashMap::with_capacity(1);
                            response_params.insert(Self::PUBLIC_KEY_PARAM_KEY, Typed::Bytes(keys.pub_key.into()));
                            let resp_packet = Packet::from_message(
                                Message::Standard(
                                    StandardMessage {
                                        flags: 0,
                                        data: Data::InternalOpResp(OperationResponse {
                                            code: req.code,
                                            return_code: 0,
                                            message: Typed::Null,
                                            params: ParameterTable::from_dict(response_params) })
                                    }), 0, true, &Default::default()).unwrap();
                            let new_self = Handshake::<Encrypted> {
                                state: Encrypted,
                            };
                            return Ok(HandshakeAnd {
                                handshake: new_self,
                                extra: (resp_packet, keys.enc),
                            });
                        } else {
                            return Err(HandshakeAnd {
                                handshake: self,
                                extra: EncryptError::MissingParameter(Self::PUBLIC_KEY_PARAM_KEY),
                            });
                        }
                    }

                }
            }
        }
        Err(HandshakeAnd {
            handshake: self,
            extra: EncryptError::UnexpectedPacket,
        })
    }
}

pub struct Encrypted;

impl Handshake<Encrypted> {
    pub fn with_auth<T: AuthProvider<E>, E>(self, auth: T) -> Handshake<Auth<T, E>> {
        Handshake {
            state: Auth {
                authenticator: auth,
                _e: Default::default(),
            }
        }
    }
}

pub trait AuthProvider<E> {
    fn validate(&mut self, params: &std::collections::HashMap<u8, Typed>) -> Result<std::collections::HashMap<u8, Typed>, E>;
}

pub struct Auth<T: AuthProvider<E>, E> {
    authenticator: T,
    _e: std::marker::PhantomData<E>,
}

#[derive(Debug)]
pub enum AuthError<E> {
    Validation(E),
    UnexpectedPacket
}

impl <T: AuthProvider<E>, E> Handshake<Auth<T, E>> {
    //const SERVER_ADDRESS_KEY: u8 = 230;
    const AUTH_REQUEST_CODE: u8 = 230;
    const USER_ID_KEY: u8 = 225;
    const NICKNAME_KEY: u8 = 225;
    pub fn authenticate(mut self, packet: &Packet, serdes_ctx: &polariton::packet::SerdesContext<(), polariton::serdes::NoCustomSerdes>) -> Result<Packet, HandshakeAnd<Auth<T, E>, AuthError<E>>> {
        if let Packet::Packet(packet) = &packet {
            if let Message::Standard(conn) = &packet.message {
                if let Data::OpReq(req) = &conn.data {
                    if req.code == Self::AUTH_REQUEST_CODE /* or 231 ???*/ {
                        let req_dict = req.params.to_owned().to_dict();
                        let mut params_resp = match self.state.authenticator.validate(&req_dict) {
                            Ok(x) => x,
                            Err(e) => {
                                return Err(HandshakeAnd {
                                    handshake: self,
                                    extra: AuthError::Validation(e),
                                });
                            }
                        };
                        if let Some(user_id) = req_dict.get(&Self::USER_ID_KEY) {
                            params_resp.insert(Self::USER_ID_KEY, user_id.to_owned());
                            params_resp.insert(Self::NICKNAME_KEY, user_id.to_owned());
                        }
                        return Ok(Packet::from_message(
                            Message::Standard(
                                StandardMessage {
                                    flags: 0,
                                    data: Data::OpResp(OperationResponse {
                                        code: Self::AUTH_REQUEST_CODE,
                                        return_code: 0,
                                        message: Typed::Null,
                                        params: params_resp.into(),
                                    })
                                }.encrypt(conn.is_encrypted())
                            ), packet.header.channel, true, serdes_ctx).unwrap());
                    }
                }
            }
        }
        Err(HandshakeAnd {
            handshake: self,
            extra: AuthError::UnexpectedPacket,
        })
    }
}
