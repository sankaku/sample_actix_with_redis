use crate::my_error::MyError;
use redis::Commands;
use std::time::Duration;

pub type R2D2Pool = r2d2::Pool<redis::Client>;
type R2d2PooledCon = r2d2::PooledConnection<redis::Client>;

const PREFIX: &str = "with_r2d2";
const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: u32 = 30;
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Creates connection pool with default settings
pub fn _simple_create_pool(host_addr: &str) -> Result<R2D2Pool, MyError> {
    let client = redis::Client::open(host_addr).map_err(|e| MyError::new_string(e.to_string()))?;

    r2d2::Pool::builder()
        .build(client)
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub fn create_pool(host_addr: &str) -> Result<R2D2Pool, MyError> {
    let client = redis::Client::open(host_addr).map_err(|e| MyError::new_string(e.to_string()))?;
    r2d2::Pool::builder()
        .max_size(MAX_POOL_SIZE)
        .connection_timeout(CONNECTION_TIMEOUT)
        .build(client)
        .map_err(|e| MyError::new_string(e.to_string()))
}

fn get_key(base: &str) -> String {
    format!("{}:{}", PREFIX, base)
}

fn create_connection(pool: &R2D2Pool) -> Result<R2d2PooledCon, MyError> {
    pool.get().map_err(|e| MyError::new_string(e.to_string()))
}

pub fn set(pool: &R2D2Pool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool)?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub fn get(pool: &R2D2Pool, key: &str) -> Result<String, MyError> {
    let mut con = create_connection(pool)?;
    let redis_key = get_key(key);
    con.get(redis_key)
        .map_err(|e| MyError::new_string(e.to_string()))
}
