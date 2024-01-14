use serde::{Serialize, Deserialize};
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

pub trait Serialization {
    fn serialize<T: Serialize>(item: &T) -> Result<Vec<u8>, Error>;
    fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, Error>;
}

pub struct Serializer;

impl Serialization for Serializer {
    fn serialize<T: Serialize>(item: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_cbor::to_vec(item)
            .map_err(|e| Error::Server(InternalServerError { context: Some(e.to_string().into()) }))?
        )
    }

    fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, Error> {
        Ok(serde_cbor::from_slice(bytes)
            .map_err(|e| Error::Server(InternalServerError { context: Some(e.to_string().into()) }))?
        )
    }
}