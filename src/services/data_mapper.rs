use serde::{Deserialize, Serialize};
use crate::errors::Error;
use crate::services::serializer::{Serialization, Serializer};

pub trait DataMapper
    where
        Self::Entity: Serialize + for<'de> Deserialize<'de>,
        Self::Schema: Serialize + for<'de> Deserialize<'de>,
{
    type Schema;
    type Entity;

    fn encode(entity: &Self::Entity) -> Result<Self::Schema, Error>
    {
        let entity = Serializer::serialize(&entity)?;

        let schema: Self::Schema = Serializer::deserialize(&entity)?;

        Ok(schema)
    }

    fn decode(schema: &Self::Schema) -> Result<Self::Entity, Error> {
        let schema = Serializer::serialize(schema)?;

        let entity: Self::Entity = Serializer::deserialize(&schema)?;

        Ok(entity)
    }
}