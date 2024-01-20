use crate::errors::Error;

pub trait DbManager: Clone {
    type Pool;
    type Connection;

    fn connect(&self, conn_str: &str) -> Result<Self, Error>;
    fn pool(&self) -> Result<Self::Pool, Error>;
    fn connection(&self) -> Result<Self::Connection, Error>;
    fn transact<R, F>(&self, f: F) -> Result<R, Error>
        where
            F: FnOnce(&mut Self::Connection) -> Result<R, Error>;
}