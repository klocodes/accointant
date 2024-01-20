use crate::errors::Error;

pub trait DbManager: Clone {
    type Pool;
    fn connect(&self, conn_str: &str) -> Result<Self, Error>;
    fn pool(&self) -> Result<Self::Pool, Error>;
}