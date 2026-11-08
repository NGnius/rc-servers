use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "provider")]
pub enum EmailConfig {
    None,
    SMTP {
        from: String,
        uri: String,
    },
}

pub fn default_email() -> EmailConfig {
    EmailConfig::None
}

impl super::config::RedactedClone for EmailConfig {
    fn redacted_clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::SMTP { from, .. } => {
                Self::SMTP {
                    from: from.to_owned(),
                    uri: "[REDACTED]".to_owned(),
                }
            },
        }
    }
}
