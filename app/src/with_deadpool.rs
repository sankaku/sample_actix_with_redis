use deadpool_redis::{
    redis::{cmd, FromRedisValue},
    Config, Connection, Manager, Pool, Runtime,
};
use redis::AsyncCommands;

use crate::my_error::MyError;

pub type DeadpoolPool = Pool; // TODO OK?
type DeadpoolConnection = Connection;

const PREFIX: &'static str = "with_deadpool";
const TTL: usize = 60 * 5;

pub fn create_pool(host_addr: &str) -> Result<DeadpoolPool, MyError> {
    let config = Config::from_url(host_addr);
    config
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|e| MyError::new_str("failed to create pool"))
}

async fn create_connection(pool: &DeadpoolPool) -> Result<DeadpoolConnection, MyError> {
    pool.get()
        .await
        .map_err(|e| MyError::new_str("failed to get connection"))
}

fn get_key(key: &str) -> String {
    format!("{}:{}", PREFIX, key)
}

pub async fn set(pool: &DeadpoolPool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool).await?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .await
        .map_err(|e| MyError::new_str("failed to set"))
}

pub async fn get(pool: &DeadpoolPool, key: &str) -> Result<String, MyError> {
    let mut con = create_connection(pool).await?;
    let redis_key = get_key(key);
    con.get(key)
        .await
        .map_err(|e| MyError::new_str("failed to get"))
}
