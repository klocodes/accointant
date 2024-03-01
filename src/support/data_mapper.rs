use serde::{Deserialize, Serialize};
use crate::services::serializer::{Serializer};
use crate::support::error::{SupportError};

pub trait DataMapper
    where
        Self::Entity: Serialize + for<'de> Deserialize<'de>,
        Self::Schema: Serialize + for<'de> Deserialize<'de>,
{
    type Schema;
    type Entity;

    fn encode (serializer: Serializer, entity: &Self::Entity) -> Result<Self::Schema, SupportError>
    {
        let entity = serializer.serialize(&entity)
            .map_err(|e| SupportError::DataMapper(e.to_string()))?;

        let schema: Self::Schema = serializer.deserialize(&entity)
            .map_err(|e| SupportError::DataMapper(e.to_string()))?;

        Ok(schema)
    }

    fn decode (serializer: Serializer, schema: &Self::Schema) -> Result<Self::Entity, SupportError> {
        let schema = serializer.serialize(schema)
            .map_err(|e| SupportError::DataMapper(e.to_string()))?;

        let entity: Self::Entity = serializer.deserialize(&schema)
            .map_err(|e| SupportError::DataMapper(e.to_string()))?;

        Ok(entity)
    }
}