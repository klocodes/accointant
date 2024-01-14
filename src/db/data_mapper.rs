use crate::errors::Error;

pub trait DataMapper {
    type Schema;
    type Entity;

    fn encode(entity: &Self::Entity) -> Result<Self::Schema, Error>;
    fn decode(schema: &Self::Schema) -> Result<Self::Entity, Error>;
}