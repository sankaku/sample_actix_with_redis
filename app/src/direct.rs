use redis::aio::Connection;
use redis::AsyncCommands;

const PREFIX: &'static str = "direct";
const TTL: usize = 60 * 5;

pub async fn create_connection() -> Result<Connection, redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_async_connection().await
}

fn get_key(key: &str) -> String {
    format!("{}:{}", PREFIX, key)
}

pub async fn set(key: &str, value: &str) -> Result<String, redis::RedisError> {
    let mut con = create_connection().await?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL).await
}
pub async fn get(key: &str) -> Result<(), redis::RedisError> {
    let mut con = create_connection().await?;
    let redis_key = get_key(key);
    con.get(redis_key).await
}

// pub async fn set_by_array(kv: [&str; 2]) -> Result<(), redis::RedisError> {
//     let mut con = create_connection().await?;
//     redis::cmd("SET").arg(&kv).query_async(&mut con).await
// }
//
// pub async fn multi_get(keys: &[&str]) -> Result<(String, Vec<u8>), redis::RedisError> {
//     let mut con = create_connection().await?;
//     redis::cmd("MGET").arg(keys).query_async(&mut con).await
// }
//
