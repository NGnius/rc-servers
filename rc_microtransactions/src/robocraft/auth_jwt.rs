use actix_web::{FromRequest, HttpRequest, dev::Payload};

#[derive(Clone)]
pub struct TokenSecret {
    secret: Box<[u8]>,
}

impl TokenSecret {
    pub async fn load(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let token_path = root.as_ref().join(oj_rc_core::persist::user::TOKEN_SECRET_FILENAME);
        let secret = tokio::fs::read(token_path).await?;
        Ok(Self {
            secret: secret.into_boxed_slice(),
        })
    }
}

#[allow(dead_code)]
pub struct PaymentToken {
    pub data: libfj::robocraft::TokenPayload,
    pub token: String,
}

#[derive(Debug)]
pub enum TokenError {
    NoHeader,
    Invalid,
    NoAuth,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHeader => write!(f, "Missing authorization token"),
            Self::Invalid => write!(f, "Invalid authorization token"),
            Self::NoAuth => write!(f, "Invalid authorization secret"),
        }
    }
}

impl actix_web::error::ResponseError for TokenError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }
}

impl std::error::Error for TokenError {}

impl FromRequest for PaymentToken {
    type Error = TokenError;
    type Future = PaymentTokenFuture;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header_opt = req.headers()
            .get(actix_web::http::header::AUTHORIZATION);
        let result = if let Some(auth_header) = auth_header_opt {
            match auth_header.to_str() {
                Ok(header_val) => {
                    if let Some((scheme, token)) = header_val.split_once(' ') {
                        match &scheme.to_lowercase() as &str {
                            "robocraft" => {
                                if let Some(secret) = req.app_data::<actix_web::web::Data<TokenSecret>>() {
                                    let secret = jsonwebtoken::DecodingKey::from_secret(&secret.secret);
                                    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
                                    validation.set_required_spec_claims::<&str>(&[]);
                                    let token_data_res = jsonwebtoken::decode::<libfj::robocraft::TokenPayload>(token, &secret, &validation);
                                    match token_data_res {
                                        Ok(data) => Ok(PaymentToken {
                                            data: data.claims,
                                            token: token.to_owned()
                                        }),
                                        Err(_e) => Err(TokenError::Invalid),
                                    }
                                } else {
                                    Err(TokenError::NoAuth)
                                }
                            },
                            _ => Err(TokenError::Invalid),
                        }
                    } else {
                        Err(TokenError::Invalid)
                    }
                },
                Err(_e) => {
                    Err(TokenError::Invalid)
                }
            }
        } else {
            Err(TokenError::NoHeader)
        };
        PaymentTokenFuture {
            result: Some(result),
        }
    }
}

pub struct PaymentTokenFuture {
    result: Option<Result<PaymentToken, TokenError>>,
}

impl core::future::Future for PaymentTokenFuture {
    type Output = Result<PaymentToken, TokenError>;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.get_mut().result
            .take()
            .map(std::task::Poll::Ready)
            .unwrap_or(std::task::Poll::Pending)
    }
}
