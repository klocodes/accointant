use serde::{Serialize, Deserialize};
use crate::services::error::ServiceError;

#[derive(Clone)]
pub enum Serializer {
    Cbor,
}

impl Serializer {
    pub fn serialize<T: Serialize>(&self, item: &T) -> Result<Vec<u8>, ServiceError> {
        let serializer =  match &self {
            Serializer::Cbor => serde_cbor::to_vec(item).map_err(|e|
                ServiceError::Serializer(e.to_string())
            )?
        };

        Ok(serializer)
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self, item: &[u8]) -> Result<T, ServiceError> {
        let deserializer = match &self {
            Serializer::Cbor => serde_cbor::from_slice(item).map_err(|e|
                ServiceError::Serializer(e.to_string())
            )?
        };

        Ok(deserializer)
    }
}