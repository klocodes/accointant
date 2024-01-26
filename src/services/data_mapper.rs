use serde::{Deserialize, Serialize};
use crate::errors::Error;
use crate::services::serializer::{Serializer};

pub trait DataMapper
    where
        Self::Entity: Serialize + for<'de> Deserialize<'de>,
        Self::Schema: Serialize + for<'de> Deserialize<'de>,
{
    type Schema;
    type Entity;

    fn encode<S: Serializer>(serializer: S, entity: &Self::Entity) -> Result<Self::Schema, Error>
    {
        let entity = serializer.serialize(&entity)?;

        let schema: Self::Schema = serializer.deserialize(&entity)?;

        Ok(schema)
    }

    fn decode<S: Serializer>(serializer: S, schema: &Self::Schema) -> Result<Self::Entity, Error> {
        let schema = serializer.serialize(schema)?;

        let entity: Self::Entity = serializer.deserialize(&schema)?;

        Ok(entity)
    }
}