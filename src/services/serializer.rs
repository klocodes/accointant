use serde::{Serialize, Deserialize};
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;


pub trait Serializer: Clone + Send + Sync {
    fn serialize<T: Serialize>(&self, item: &T) -> Result<Vec<u8>, Error>;
    fn deserialize<'a, T: Deserialize<'a>>(&self, bytes: &'a [u8]) -> Result<T, Error>;
}

#[derive(Clone)]
pub struct CborSerializer;

impl CborSerializer {
    pub fn new() -> Self {
        Self
    }
}

impl Serializer for CborSerializer {
    fn serialize<T: Serialize>(&self, item: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_cbor::to_vec(item)
            .map_err(|e| Error::Server(InternalServerError { context: Some(e.to_string().into()) }))?
        )
    }

    fn deserialize<'a, T: Deserialize<'a>>(&self, bytes: &'a [u8]) -> Result<T, Error> {
        Ok(serde_cbor::from_slice(bytes)
            .map_err(|e| Error::Server(InternalServerError { context: Some(e.to_string().into()) }))?
        )
    }
}