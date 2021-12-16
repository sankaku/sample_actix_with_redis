use std::time::Duration;

use deadpool_redis::{Config, Connection, Pool, Runtime};
use redis::AsyncCommands;

use crate::my_error::MyError;

pub type DeadpoolPool = Pool;
type DeadpoolConnection = Connection;

const PREFIX: &str = "with_deadpool";
const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: usize = 30;
const WAIT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

/// Creates connection pool with default settings
pub fn _simple_create_pool(host_addr: &str) -> Result<DeadpoolPool, MyError> {
    let config = Config::from_url(host_addr);
    config
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub fn create_pool(host_addr: &str) -> Result<DeadpoolPool, MyError> {
    let config = Config::from_url(host_addr);
    config
        .builder()
        .map(|b| {
            b.max_size(MAX_POOL_SIZE)
                .wait_timeout(WAIT_TIMEOUT) // TODO needs create_timeout/recycle timeout?
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap() // TODO don't panic. flat_map can't be used???
        })
        .map_err(|e| MyError::new_string(e.to_string()))
}

async fn create_connection(pool: &DeadpoolPool) -> Result<DeadpoolConnection, MyError> {
    pool.get()
        .await
        .map_err(|e| MyError::new_string(e.to_string()))
}

fn get_key(key: &str) -> String {
    format!("{}:{}", PREFIX, key)
}

pub async fn set(pool: &DeadpoolPool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool).await?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .await
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub async fn get(pool: &DeadpoolPool, key: &str) -> Result<String, MyError> {
    let mut con = create_connection(pool).await?;
    let redis_key = get_key(key);
    con.get(redis_key)
        .await
        .map_err(|e| MyError::new_string(e.to_string()))
}
