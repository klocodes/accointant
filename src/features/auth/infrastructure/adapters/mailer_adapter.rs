use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::services::mailer::Mailer;

pub struct MailerAdapter<M: Mailer> {
    mailer: M,
}

impl<M: Mailer> MailerAdapter<M> {
    pub fn new(mailer: M) -> Self {
        MailerAdapter { mailer }
    }

    pub async fn send(&self, to: String, subject: String, body: String) -> Result<(), AuthError> {
        self.mailer.send(to, subject, body)
            .await.
            map_err(|e| AuthError::Infrastructure(
                InfrastructureError::Mailer(e.to_string())
            ))
    }
}