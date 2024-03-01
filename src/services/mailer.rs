use async_trait::async_trait;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::address::AddressError;
use lettre::message::{Mailbox, SinglePart};
use lettre::message::header::ContentType;
use lettre::transport::smtp::client::Tls;
use mockall::automock;
use crate::config::structs::mailer::MailerConfig;
use crate::services::error::ServiceError;


#[automock]
#[async_trait]
pub trait Mailer {
    async fn send(&self, email: String, subject: String, body: String) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct LettreMailer {
    host: String,
    port: u16,
    username: String,
    password: String,
    from: String,
}

impl LettreMailer {
    pub fn new(cfg: &MailerConfig) -> Self {
        Self {
            host: cfg.host().to_string(),
            port: cfg.port(),
            username: cfg.username().to_string(),
            password: cfg.password().to_string(),
            from: cfg.from().to_string(),
        }
    }
}

#[async_trait]
impl Mailer for LettreMailer {
    async fn send(&self, email: String, subject: String, body: String) -> Result<(), ServiceError> {
        let from: Mailbox = self.from.parse().map_err(|e: AddressError| {
            ServiceError::Mailer(e.to_string())
        })?;
        let email: Mailbox = email.parse().map_err(|e: AddressError| {
            ServiceError::Mailer(e.to_string())
        })?;

        let single_part = SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(body);

        let message = Message::builder()
            .from(from.clone())
            .to(email.clone())
            .subject(subject)
            .singlepart(single_part)
            .map_err(|e| {
                ServiceError::Mailer(e.to_string())
            })?;

        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)
            .map_err(|e| {
                ServiceError::Mailer(e.to_string())
            })?
            .port(self.port)
            .tls(Tls::None)
            .build();

        mailer.send(message)
            .await
            .map_err(|e| {
                ServiceError::Mailer(e.to_string())
            })?;

        Ok(())
    }
}