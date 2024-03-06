use serde::{Serialize, Deserialize};
use crate::services::error::ServiceError;

#[derive(Clone)]
pub enum Serializer {
    Cbor,
    Json,
}

impl Serializer {
    pub fn serialize<T>(&self, item: &T) -> Result<Vec<u8>, ServiceError>
        where
            T: Serialize,
    {
        match &self {
            Serializer::Cbor => serde_cbor::to_vec(item).map_err(|e| ServiceError::Serializer(e.to_string())),
            Serializer::Json => {
                let json = serde_json::to_string(item).map_err(|e| ServiceError::Serializer(e.to_string()))?;
                Ok(json.into_bytes())
            }
        }
    }

    pub fn deserialize<'de, T: Deserialize<'de>>(&self, item: &'de [u8]) -> Result<T, ServiceError> {
        match &self {
            Serializer::Cbor => serde_cbor::from_slice(item).map_err(|e| ServiceError::Serializer(e.to_string())),
            Serializer::Json => serde_json::from_slice(item).map_err(|e| ServiceError::Serializer(e.to_string())),
        }
    }
}
