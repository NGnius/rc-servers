use lettre::AsyncTransport;

pub struct EmailSender {
    db: std::sync::Arc<oj_rc_database::Database>,
    implementor: EmailSenderImpl,
}

impl EmailSender {
    /// This will panic on bad config
    pub fn new(config: &crate::ConfigImpl, user_impl: &crate::UserImpl) -> Self {
        let config = config.email_conf();
        let db = user_impl.db_impl();
        let variant = match config {
            crate::persist::EmailConfig::None => EmailSenderImpl::Noop,
            crate::persist::EmailConfig::SMTP { from, uri } => EmailSenderImpl::Smtp(Box::new(SmtpImpl::new(from, uri))),
        };
        Self {
            db,
            implementor: variant,
        }
    }

    pub async fn send_email(&self, to: crate::types::AccountId, subject: String, message: String) -> Result<(), Box<dyn core::error::Error>> {
        if let Ok(Some(account)) = crate::persist::user::find_user(self.db.as_ref(), &to).await {
            if account.email.contains('@') {
                self.implementor.send_email_to(account.display_name, account.email, subject, message).await
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub async fn test(&self) -> Result<bool, Box<dyn core::error::Error>> {
        self.implementor.test().await
    }
}

enum EmailSenderImpl {
    Noop,
    Smtp(Box<SmtpImpl>)
}

impl EmailSenderImpl {
    async fn send_email_to(&self, display_name: String, address: String, subject: String, message: String) -> Result<(), Box<dyn core::error::Error>> {
        match self {
            Self::Noop => Err("Email is not configured".into()),
            Self::Smtp(smtp) => smtp.send_email_to(display_name, address, subject, message).await
        }
    }

    async fn test(&self) -> Result<bool, Box<dyn core::error::Error>> {
        match self {
            Self::Noop => {
                log::info!("Email self-test is a no-op (email not configured)");
                Ok(true)
            },
            Self::Smtp(smtp) => smtp.test().await
        }
    }
}

struct SmtpImpl {
    from: lettre::message::Mailbox,
    sender: lettre::AsyncSmtpTransport<lettre::Tokio1Executor>,
}

impl SmtpImpl {
    /// This will panic on bad config
    pub(crate) fn new(from: &str, uri: &str) -> Self {
        let sender = lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::from_url(uri)
            .expect("Bad email uri")
            .build();
        let from_addr = from.parse().expect("Bad email from address");
        Self {
            from: from_addr,
            sender,
        }
    }

    async fn send_email_to(&self, display_name: String, address: String, subject: String, message: String) -> Result<(), Box<dyn core::error::Error>> {
        let address: lettre::message::Mailbox = address.parse()?;
        let message = lettre::message::MessageBuilder::new()
            .from(self.from.clone())
            .to(lettre::message::Mailbox {
                name: Some(display_name),
                email: address.email,
            })
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(message)?;
        self.sender.send(message).await?;
        Ok(())
    }

    async fn test(&self) -> Result<bool, Box<dyn core::error::Error>> {
        Ok(self.sender.test_connection().await?)
    }
}


