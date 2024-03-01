use chrono::{Duration, Utc};
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::adapters::hasher_adapter::HasherAdapter;
use crate::features::auth::infrastructure::adapters::jwt_adapter::JwtServiceAdapter;
use crate::services::hasher::Hasher;
use crate::services::jwt::{Claims, JwtService};
use crate::support::error::FeatureError;

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
        _hasher: HasherAdapter<impl Hasher>,
        jwt_adapter: JwtServiceAdapter<impl JwtService>,
        rep: impl UserRepository,
    ) -> Result<String, FeatureError>
    {
        let user = rep.find_by_email(self.email.clone())
            .await
            .map_err(|e| FeatureError::Auth(e))?
            .ok_or(
                FeatureError::Auth(
                    AuthError::Domain(
                        DomainError::EmailNotFound
                    )
                )
            )?;

        if user.confirmed_at().is_none() {
            return Err(
                FeatureError::Auth(
                    AuthError::Domain(
                        DomainError::EmailHasNotConfirmed
                    )
                )
            );
        }

        let claims = Claims::new(
            user.id().to_string(),
            Utc::now().timestamp() as usize + Duration::days(30).num_seconds() as usize,
            user.email().value().to_string(),
        );
        let token = jwt_adapter.create(claims).map_err(|e| FeatureError::Auth(e))?;

        Ok(token)
    }
}

