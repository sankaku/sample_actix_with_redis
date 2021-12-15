use redis::aio::Connection;
use redis::{AsyncCommands, Client};

use crate::my_error::MyError;

pub type DirectClient = Client;

const PREFIX: &str = "direct";
const TTL: usize = 60 * 5;

pub async fn create_client(host_addr: &str) -> Result<DirectClient, MyError> {
    redis::Client::open(host_addr).map_err(|_| MyError::new_str("failed to create client"))
}

pub async fn create_connection(client: &DirectClient) -> Result<Connection, MyError> {
    client
        .get_async_connection()
        .await
        .map_err(|e| MyError::new_str("failed to create connection"))
}

fn get_key(key: &str) -> String {
    format!("{}:{}", PREFIX, key)
}

pub async fn set(client: &DirectClient, key: &str, value: &str) -> Result<String, MyError> {
    let mut con = create_connection(client).await?;
    let redis_key = get_key(key);
    con.set_ex(redis_key, value, TTL)
        .await
        .map_err(|e| MyError::new_str("failed to set"))
}
pub async fn get(client: &DirectClient, key: &str) -> Result<(), MyError> {
    let mut con = create_connection(client).await?;
    let redis_key = get_key(key);
    con.get(redis_key)
        .await
        .map_err(|e| MyError::new_str("failed to set"))
}
