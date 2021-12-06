use deadpool_redis::{
    redis::{cmd, FromRedisValue},
    Config,
    Connection,
    Manager,
    Runtime,
    Pool
};
use redis::AsyncCommands;

use crate::my_error::MyError;

pub type DeadpoolConnection = Pool; // TODO OK?

pub fn create_pool(host_addr: &str) -> Result<DeadpoolConnection, MyError> {
    let config = Config::from_url(host_addr);
    config.create_pool(Some(Runtime::Tokio1))
    .map_err(|e| MyError::new_str("failed to create pool"))
}

// TODO not &pool???
pub async fn set(pool: DeadpoolConnection, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = pool.get().await.map_err(|e| MyError::new_str("failed to get connection"))?;
    con.set(key, value).await.map_err(|e| MyError::new_str("failed to set"))
}
