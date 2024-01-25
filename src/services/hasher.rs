use mockall::automock;
use crate::errors::Error;

#[automock]
pub trait Hasher {
    fn hash(&self, value: String) -> Result<String, Error>;
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
}