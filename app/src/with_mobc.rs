use mobc::Connection;
use mobc::Pool;
use mobc_redis::redis;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;

use crate::my_error::MyError;
use std::time::Duration;

pub type MobcPool = Pool<mobc_redis::RedisConnectionManager>;
type MobcConnection = Connection<RedisConnectionManager>;

const PREFIX: &str = "with_mobc";
const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: u64 = 30;
const CONNECTION_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

/// Creates connection pool with default settings
pub fn _simple_create_pool(host_addr: &str) -> MobcPool {
    let client = redis::Client::open(host_addr).unwrap();
    let manager = RedisConnectionManager::new(client);
    Pool::builder().build(manager)
}

pub fn create_pool(host_addr: &str) -> MobcPool {
    let client = redis::Client::open(host_addr).unwrap();
    let manager = RedisConnectionManager::new(client);
    Pool::builder()
        .max_open(MAX_POOL_SIZE)
        .get_timeout(CONNECTION_TIMEOUT) // this method SETs timeout
        .build(manager)
}

async fn create_connection(pool: &MobcPool) -> Result<MobcConnection, MyError> {
    pool.get()
        .await
        .map_err(|e| MyError::new_string(e.to_string()))
}

fn get_key(base: &str) -> String {
    format!("{}:{}", PREFIX, base)
}

pub async fn set(pool: &MobcPool, key: &str, value: &str) -> Result<(), MyError> {
    let redis_key = get_key(key);
    let mut con = create_connection(pool).await?;
    con.set_ex(redis_key, value, TTL)
        .await
        .map_err(|e| MyError::new_string(e.to_string()))
}

pub async fn get(pool: &MobcPool, key: &str) -> Result<String, MyError> {
    let redis_key = get_key(key);
    let mut con = create_connection(pool).await?;
    con.get(redis_key)
        .await
        .map_err(|e| MyError::new_string(e.to_string()))
}
