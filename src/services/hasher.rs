use mockall::automock;
use crate::services::error::ServiceError;

#[automock]
pub trait Hasher {
    fn hash(&self, value: String) -> Result<String, ServiceError>;

    fn verify(&self, value: String, hash: &str) -> Result<bool, ServiceError>;
}

pub struct BcryptHasher;

impl BcryptHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Hasher for BcryptHasher {
    fn hash(&self, value: String) -> Result<String, ServiceError> {
        bcrypt::hash(value, bcrypt::DEFAULT_COST)
            .map_err(|e| {
                ServiceError::Hasher(
                    format!("Failed to hash: {}", e.to_string())
                )
            })
    }

    fn verify(&self, value: String, hash: &str) -> Result<bool, ServiceError> {
        let res = bcrypt::verify(value, hash)
            .map_err(|e| {
                ServiceError::Hasher(
                    format!("Failed to verify: {}", e.to_string())
                )
            })?;

        Ok(res)
    }
}