use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::services::jwt::{Claims, JwtService};

pub struct JwtServiceAdapter<S: JwtService> {
    jwt_service: S,
}

impl<S: JwtService> JwtServiceAdapter<S> {
    pub fn new(jwt_service: S) -> Self {
        JwtServiceAdapter { jwt_service }
    }

    pub fn create(&self, claims: Claims) -> Result<String, AuthError> {
        self.jwt_service.create(claims)
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Jwt(e.to_string())
                )
            )
    }
}