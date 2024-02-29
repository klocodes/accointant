use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::services::hasher::Hasher;

pub struct HasherAdapter<H: Hasher> {
    hasher: H,
}

impl<H: Hasher> HasherAdapter<H> {
    pub fn new(hasher: H) -> Self {
        HasherAdapter { hasher }
    }

    pub fn hash(&self, password: String) -> Result<String, AuthError> {
        self.hasher.hash(password)
            .map_err(|e| AuthError::Infrastructure(
                InfrastructureError::Hasher(e.to_string())
            ))
    }

    pub fn verify(&self, password: String, hash: &str) -> Result<bool, AuthError> {
        self.hasher.verify(password, hash)
            .map_err(|e| AuthError::Infrastructure(
                InfrastructureError::Hasher(e.to_string())
            ))
    }
}