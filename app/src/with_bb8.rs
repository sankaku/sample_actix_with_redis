use crate::my_error::MyError;
use bb8_redis::{
    bb8::{Pool, PooledConnection},
    redis::AsyncCommands,
    RedisConnectionManager,
};

pub type BB8Pool = Pool<RedisConnectionManager>;
// type BB8Connetion = &'static mut PooledConnection<'static, RedisConnectionManager>; // lifetime???

const PREFIX: &'static str = "with_bb8";
const TTL: usize = 60 * 5;

pub async fn create_pool(host_addr: &str) -> Result<BB8Pool, MyError> {
    let manager = RedisConnectionManager::new(host_addr)
        .map_err(|_| MyError::new_str("Failed to create connection manager"))?;
    Pool::builder()
        .build(manager)
        .await
        .map_err(|_| MyError::new_str("failed to create connection pool"))
}

fn get_key(key: &str) -> String {
    format!("{}:{}", PREFIX, key)
}

// async fn create_connection(pool: &BB8Pool) -> Result<BB8Connetion, MyError> {
//     pool
//         .get()
//         .await
//         .map_err(|_| MyError::new_str("failed to get connection"))
// }

pub async fn set(pool: &BB8Pool, key: &str, value: &str) -> Result<(), MyError> {
    let mut con = pool
        .get()
        .await
        .map_err(|_| MyError::new_str("failed to get connection"))?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .await
        .map_err(|_| MyError::new_str("failed to set"))
}

pub async fn get(pool: &BB8Pool, key: &str) -> Result<String, MyError> {
    let mut con = pool
        .get()
        .await
        .map_err(|_| MyError::new_str("failed to get connection"))?;
    let redis_key = get_key(key);
    con.get(redis_key)
        .await
        .map_err(|_| MyError::new_str("failed to get"))
}
