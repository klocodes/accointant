use async_trait::async_trait;
use futures_util::TryStreamExt;
use lettre::{AsyncSmtpTransport, AsyncTransport, Executor, Message, Tokio1Executor, Transport};
use lettre::address::AddressError;
use lettre::message::Mailbox;
use lettre::transport::smtp::client::Tls;
use crate::config::mailer::MailerConfig;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[async_trait]
pub trait Mailer {
    fn new(cfg: &MailerConfig) -> Self;
    async fn send(&self, email: String, subject: String, body: String) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct LettreMailer {
    host: String,
    port: u16,
    username: String,
    password: String,
    from: String,
}

#[async_trait]
impl Mailer for LettreMailer {
    fn new(cfg: &MailerConfig) -> Self {
        Self {
            host: cfg.host().to_string(),
            port: cfg.port(),
            username: cfg.username().to_string(),
            password: cfg.password().to_string(),
            from: cfg.from().to_string(),
        }
    }

    async fn send(&self, email: String, subject: String, body: String) -> Result<(), Error> {
        let from: Mailbox = self.from.parse().map_err(|e: AddressError| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to parse email: {}", e.to_string()).into()
                    )
                }
            )
        })?;
        let email: Mailbox  = email.parse().map_err(|e: AddressError| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to parse email: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        let message = Message::builder()
            .from(from.clone())
            .to(email.clone())
            .subject(subject)
            .body(body)
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to build message: {}", e.to_string()).into()
                        )
                    }
                )
            })?;

        println!("email: {}, from: {}, message: {:?}", email, from, message);

        // Настройте SMTP клиент для MailHog
        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to relay mailer: {}", e.to_string()).into()
                        )
                    }
                )
            })?
            .port(self.port)
            .tls(Tls::None) // Указываем, что TLS не используется
            .build();

        // Отправьте письмо
        mailer.send(message)
            .await
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to send email: {}", e.to_string()).into()
                        )
                    }
                )
            })?;

        Ok(())
    }
}