use crate::db::data_mapper::DataMapper;
use crate::errors::Error;
use crate::features::auth::domain::user::User;
use crate::features::auth::infrastructure::mapping::user_schema::UserSchema;
use crate::service::serializer::{Serialization, Serializer};

pub struct UserMapper;

impl DataMapper for UserMapper {
    type Schema = UserSchema;
    type Entity = User;

    fn encode(entity: &Self::Entity) -> Result<Self::Schema, Error> {
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