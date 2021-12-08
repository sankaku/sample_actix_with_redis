use mobc::Pool;
use mobc_redis::redis;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;

use crate::my_error::MyError;

pub type MobcPool = Pool<mobc_redis::RedisConnectionManager>;

pub fn create_pool(host_addr: &str) -> MobcPool {
    let client = redis::Client::open(host_addr).unwrap();
    let manager = RedisConnectionManager::new(client);
    Pool::builder().max_open(10).build(manager)
}

pub async fn set(pool: &MobcPool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = pool
        .get()
        .await
        .map_err(|e| MyError::new_str("failed to create connection"))?;
    con.set(key, value)
        .await
        .map_err(|e| MyError::new_str("failed to set"))
}
