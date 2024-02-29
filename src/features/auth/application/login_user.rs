use chrono::{Duration, Utc};
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::adapters::hasher_adapter::HasherAdapter;
use crate::features::auth::infrastructure::adapters::jwt_adapter::JwtServiceAdapter;
use crate::services::hasher::Hasher;
use crate::services::jwt::{Claims, JwtService};

pub struct LoginUser {
    email: String,
    password: String,
}

impl LoginUser {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub async fn exec(
        &self,
        hasher: HasherAdapter<impl Hasher>,
        jwt_adapter: JwtServiceAdapter<impl JwtService>,
        rep: impl UserRepository,
    ) -> Result<String, AuthError>
    {
        let user = rep.find_by_email(self.email.clone())
            .await?
            .ok_or(
                AuthError::Domain(
                    DomainError::EmailNotFound
                )
            )?;

        if user.confirmed_at().is_none() {
            return Err(
                AuthError::Domain(
                    DomainError::EmailHasNotConfirmed
                )
            );
        }

        if !hasher.verify(self.password.clone(), user.password().value())? {
            return Err(
                AuthError::Domain(
                    DomainError::WrongPassword
                )
            );
        }

        let claims = Claims::new(
            user.id().to_string(),
            Utc::now().timestamp() as usize + Duration::days(30).num_seconds() as usize,
            user.email().value().to_string(),
        );
        let token = jwt_adapter.create(claims)?;

        Ok(token)
    }
}

