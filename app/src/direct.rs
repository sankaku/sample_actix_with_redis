use redis::aio::Connection;
use redis::AsyncCommands;

pub async fn create_connection() -> Result<Connection, redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_async_connection().await
}

pub async fn set(key: &str, value: &str) -> Result<(), redis::RedisError> {
    let mut con = create_connection().await?;
    con.set(key, value).await
}

pub async fn set_by_array(kv: [&str; 2]) -> Result<(), redis::RedisError> {
    let mut con = create_connection().await?;
    redis::cmd("SET").arg(&kv).query_async(&mut con).await
}

pub async fn multi_get(keys: &[&str]) -> Result<(String, Vec<u8>), redis::RedisError> {
    let mut con = create_connection().await?;
    redis::cmd("MGET").arg(keys).query_async(&mut con).await
}
