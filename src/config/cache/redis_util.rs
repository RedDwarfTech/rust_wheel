use std::env;
use redis::{Commands, Connection, FromRedisValue, Value};
use crate::config::cache::mobc_error::Error;
use crate::config::cache::mobc_error::MobcError::{RedisClientError, RedisCMDError, RedisTypeError};

type Result<T> = std::result::Result<T, Error>;

pub fn get_con(client: redis::Client) -> Result<Connection> {
    client
        .get_connection()
        .map_err(|e| RedisClientError(e).into())
}

pub async fn set_str(con: &mut Connection, key: &str, value: &str, ttl_seconds: usize) -> Result<()> {
    con.set(key, value).map_err(RedisCMDError)?;
    if ttl_seconds > 0 {
        con.expire(key, ttl_seconds).map_err(RedisCMDError)?;
    }
    Ok(())
}

pub async fn get_str_default(key: &str) -> Result<String>{
    let config_redis_string = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_con_string: &str = config_redis_string.as_str();
    let redis_client = redis::Client::open(redis_con_string).expect("can create redis client");
    let mut redis_conn = get_con(redis_client);
    let value = redis_conn.unwrap().get(key).map_err(RedisCMDError)?;
    if Value::Nil == value {
        return Ok("null".parse().unwrap());
    }
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}

pub async fn get_str(con: &mut Connection, key: &str) -> Result<String> {
    let value = con.get(key).map_err(RedisCMDError)?;
    if Value::Nil == value {
        return Ok("null".parse().unwrap());
    }
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}

pub async fn del_redis_key(key: &str) -> Result<()> {
    let config_redis_string = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_con_string: &str = config_redis_string.as_str();
    let redis_client = redis::Client::open(redis_con_string).expect("can create redis client");
    let redis_conn = get_con(redis_client);
    let mut redis_conn_unwrap = redis_conn.unwrap();
    let del_result = redis_conn_unwrap.del(key).map_err(RedisCMDError)?;
    FromRedisValue::from_redis_value(&del_result).map_err(|e| RedisTypeError(e).into())
}

