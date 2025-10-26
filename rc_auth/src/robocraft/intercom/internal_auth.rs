pub struct IntercomAuth {
    key: Vec<u8>,
}

impl IntercomAuth {
    pub fn new(data: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let key = std::fs::read(data.as_ref().join(oj_rc_core::persist::user::TOKEN_SECRET_FILENAME))?;
        Ok(Self {
            key,
        })
    }

    fn validate_token(&self, received_token: &str, salt: &str) -> Result<(), IntercomOpError> {
        let expected_token = oj_rc_core::persist::user::generate_intercom_token(salt.as_bytes(), &self.key);
        if received_token.to_lowercase() == expected_token.to_lowercase() {
            Ok(())
        } else {
            Err(IntercomOpError::Unauthorized)
        }
    }

    pub fn validate(&self, req: &actix_web::HttpRequest, salt: &str) -> Result<(), IntercomOpError> {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(header_val) = auth_header.to_str() {
                if let Some((bearer, token)) = header_val.split_once(" ") {
                    if bearer.to_lowercase() == "internal" || bearer.to_lowercase() == "bearer" {
                        self.validate_token(token, salt)?;
                    } else {
                        return Err(IntercomOpError::Unauthorized);
                    }
                } else {
                    return Err(IntercomOpError::BadHeader);
                }
            } else {
                return Err(IntercomOpError::BadHeader);
            }
        } else {
            return Err(IntercomOpError::Unauthorized);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum IntercomOpError {
    BadHeader,
    Unauthorized,
    #[allow(dead_code)]
    Io(std::io::Error),
    #[allow(dead_code)]
    Unknown,
}

impl core::fmt::Display for IntercomOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Intercom error variant {:?}", self)
    }
}

impl actix_web::error::ResponseError for IntercomOpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::BadHeader => actix_web::http::StatusCode::BAD_REQUEST,
            Self::Unauthorized => actix_web::http::StatusCode::FORBIDDEN,
            Self::Io(_) => actix_web::http::StatusCode::INSUFFICIENT_STORAGE,
            Self::Unknown => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::Io(io_e) => {
                actix_web::HttpResponse::new(self.status_code())
                    .set_body(format!("Intercom IO error: {}", io_e))
                    .map_into_boxed_body()
            },
            _ => {
                actix_web::HttpResponse::new(self.status_code()).set_body(self.to_string()).map_into_boxed_body()
            }
        }

    }
}
