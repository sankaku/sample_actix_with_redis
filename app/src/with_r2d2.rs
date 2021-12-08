extern crate r2d2_redis;

use crate::my_error::MyError;
use r2d2_redis::r2d2::PooledConnection;
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};

pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;
type PooledCon = PooledConnection<RedisConnectionManager>;

const PREFIX: &'static str = "with_r2d2";
const TTL: usize = 60 * 5;

pub fn create_pool(host_addr: &str) -> Result<R2D2Pool, MyError> {
    let manager = RedisConnectionManager::new(host_addr)
        .map_err(|_| MyError::new_str("failed to create manager"))?;
    let pool = r2d2::Pool::builder()
        .build(manager)
        .map_err(|_| MyError::new_str("failed to create pool"));
    pool
}

fn get_key(base: &str) -> String {
    format!("{}:{}", PREFIX, base)
}

fn create_connection(pool: &R2D2Pool) -> Result<PooledCon, MyError> {
    pool.get()
        .map_err(|_| MyError::new_str("failed to create connection}"))
}

pub fn set(pool: &R2D2Pool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool)?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub fn get(pool: &R2D2Pool, key: &str) -> Result<(), MyError> {
    let mut con = create_connection(pool)?;
    let redis_key = get_key(key);
    con.get(redis_key)
        .map_err(|e| MyError::new_string(e.to_string()))
}
