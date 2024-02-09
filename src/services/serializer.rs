use serde::{Serialize, Deserialize};
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;


#[derive(Clone)]
pub enum Serializer {
    Cbor,
}

impl Serializer {
    pub fn serialize<T: Serialize>(&self, item: &T) -> Result<Vec<u8>, Error> {
        let serializer =  match &self {
            Serializer::Cbor => serde_cbor::to_vec(item).map_err(
                |e| Error::Server(InternalServerError { context: Some(e.to_string().into()) })
            )?
        };

        Ok(serializer)
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self, item: &[u8]) -> Result<T, Error> {
        let deserializer = match &self {
            Serializer::Cbor => serde_cbor::from_slice(item).map_err(
                |e| Error::Server(InternalServerError { context: Some(e.to_string().into()) })
            )?
        };

        Ok(deserializer)
    }
}