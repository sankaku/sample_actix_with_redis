use crate::my_error::MyError;
use bb8_redis::{
    bb8::Pool,
    redis::{cmd, AsyncCommands, RedisError},
    RedisConnectionManager,
};

pub type BB8Pool = Pool<RedisConnectionManager>;

pub async fn create_pool(host_addr: &str) -> Result<Pool<RedisConnectionManager>, MyError> {
    let manager = RedisConnectionManager::new(host_addr)
        .map_err(|_| MyError::new_str("Failed to create connection manager"))?;
    Pool::builder()
        .build(manager)
        .await
        .map_err(|_| MyError::new_str("failed to create connection pool"))
}

pub async fn set(pool: &BB8Pool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = pool
        .get()
        .await
        .map_err(|_| MyError::new_str("failed to get connection"))?;
    con.set(key, value)
        .await
        .map_err(|_| MyError::new_str("failed to set"))
}
