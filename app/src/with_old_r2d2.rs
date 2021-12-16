use crate::my_error::MyError;
use r2d2_redis::r2d2::PooledConnection;
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::time::Duration;

pub type OldR2D2Pool = r2d2::Pool<RedisConnectionManager>;
type OldR2D2PooledCon = PooledConnection<RedisConnectionManager>;

const PREFIX: &str = "with_old_r2d2";
const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: u32 = 30;
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Creates connection pool with default settings
pub fn _simple_create_pool(host_addr: &str) -> Result<OldR2D2Pool, MyError> {
    let manager =
        RedisConnectionManager::new(host_addr).map_err(|e| MyError::new_string(e.to_string()))?;
    let pool = r2d2::Pool::builder()
        .build(manager)
        .map_err(|e| MyError::new_string(e.to_string()));
    pool
}

pub fn create_pool(host_addr: &str) -> Result<OldR2D2Pool, MyError> {
    let manager =
        RedisConnectionManager::new(host_addr).map_err(|e| MyError::new_string(e.to_string()))?;
    let pool = r2d2::Pool::builder()
        .max_size(MAX_POOL_SIZE)
        .connection_timeout(CONNECTION_TIMEOUT)
        .build(manager)
        .map_err(|e| MyError::new_string(e.to_string()));
    pool
}

fn get_key(base: &str) -> String {
    format!("{}:{}", PREFIX, base)
}

fn create_connection(pool: &OldR2D2Pool) -> Result<OldR2D2PooledCon, MyError> {
    pool.get().map_err(|e| MyError::new_string(e.to_string()))
}

pub fn set(pool: &OldR2D2Pool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool)?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub fn get(pool: &OldR2D2Pool, key: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool)?;
    let redis_key = get_key(key);
    con.get(redis_key)
        .map_err(|e| MyError::new_string(e.to_string()))
}
