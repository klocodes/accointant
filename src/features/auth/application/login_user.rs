use chrono::{Duration, Utc};
use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::features::auth::domain::user_repository::UserRepository;
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
        hasher: impl Hasher,
        jwt_service: impl JwtService,
        rep: impl UserRepository,
    ) -> Result<String, Error>
    {
        let user = rep.find_by_email(self.email.clone()).await?.ok_or(
            Error::Client(
                BadRequest {
                    message: Some("Email is invalid".into())
                }
            )
        )?;

        if user.confirmed_at().is_none() {
            return Err(
                Error::Client(
                    BadRequest {
                        message: Some("Email is not confirmed".into())
                    }
                )
            );
        }

        if !hasher.verify(self.password.clone(), user.password().value())? {
            return Err(
                Error::Client(
                    BadRequest {
                        message: Some("Password is invalid".into())
                    }
                )
            );
        }

        let claims = Claims::new(
            user.id().to_string(),
            Utc::now().timestamp() as usize + Duration::days(30).num_seconds() as usize,
            user.email().value().to_string(),
        );
        let token = jwt_service.create(claims)?;

        Ok(token)
    }
}

