use mockall::automock;
use crate::errors::Error;

#[automock]
pub trait Hasher {
    fn hash(&self, value: String) -> Result<String, Error>;

    fn verify(&self, value: String, hash: &str) -> Result<bool, Error>;
}

pub struct BcryptHasher;

impl BcryptHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Hasher for BcryptHasher {
    fn hash(&self, value: String) -> Result<String, Error> {
        bcrypt::hash(value, bcrypt::DEFAULT_COST)
            .map_err(|e| {
                Error::Server(
                    crate::errors::server::ServerErrors::InternalServerError {
                        context: Some(
                            format!("Failed to hash: {}", e.to_string()).into()
                        )
                    }
                )
            })
    }

    fn verify(&self, value: String, hash: &str) -> Result<bool, Error> {
        let res = bcrypt::verify(value, hash)
            .map_err(|e| {
                Error::Server(
                    crate::errors::server::ServerErrors::InternalServerError {
                        context: Some(
                            format!("Failed to verify hash: {}", e.to_string()).into()
                        )
                    }
                )
            })?;

        Ok(res)
    }
}