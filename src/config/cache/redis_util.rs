use crate::config::cache::mobc_error::Error;
use crate::config::cache::mobc_error::MobcError::{RedisCMDError, RedisTypeError};
use log::error;
use redis::{Commands, Connection, FromRedisValue, Value};
use std::env;

use super::mobc_error::MobcError;

pub fn get_con() -> Connection {
    let config_redis_string: String = env::var("REDIS_URL").expect("redis url missing");
    let redis_con_string: &str = config_redis_string.as_str();
    let redis_client = redis::Client::open(redis_con_string).expect("get redis client failed");
    let con = redis_client
        .get_connection()
        .expect("get redis connection failed");
    return con;
}

pub fn set_str(con: &mut Connection, key: &str, value: &str, ttl_seconds: usize) {
    let set_result: Result<i32, MobcError> = con.set(key, value).map_err(RedisCMDError);
    match set_result {
        Ok(_) => {
            if ttl_seconds > 0 {
                let expire_result = con
                    .expire::<&str, usize>(key, ttl_seconds)
                    .map_err(RedisCMDError);
                match expire_result {
                    Ok(_) => {}
                    Err(_) => {
                        error!("expire redis {} value error", key);
                    }
                }
            }
        }
        Err(_) => {
            error!("set redis {} value error", key);
        }
    }
}

pub fn get_str_default(key: &str) -> Result<String, Error> {
    let mut redis_conn = get_con();
    let value = redis_conn.get(key).map_err(RedisCMDError)?;
    if Value::Nil == value {
        return Ok("null".parse().unwrap());
    }
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}

pub async fn get_str(con: &mut Connection, key: &str) -> Result<String, Error> {
    let value = con.get(key).map_err(RedisCMDError)?;
    if Value::Nil == value {
        return Ok("null".parse().unwrap());
    }
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}

pub async fn del_redis_key(key: &str) -> Result<(), Error> {
    let mut redis_conn_unwrap = get_con();
    let del_result = redis_conn_unwrap.del(key).map_err(RedisCMDError)?;
    FromRedisValue::from_redis_value(&del_result).map_err(|e| RedisTypeError(e).into())
}

pub fn sync_get_str(key: &str) -> redis::RedisResult<Option<String>> {
    let mut connection = get_con();
    return connection.get(key);
}

pub fn get_list_size(key: &str) -> Result<usize, Error> {
    let mut connection = get_con();
    let size = connection.llen(key).unwrap();
    Ok(size)
}

pub fn push_to_stream(stream_key: &str) {
    let mut connection = get_con();
    let result = connection.xadd::<&str, &str, &str, &str, String>(
        stream_key,
        "*",
        &[("id", "2"), ("sub_source_id", "3")],
    );
    match result {
        Ok(_) => {}
        Err(e) => {
            error!("Couldn't send to redis,{}",e);
        }
    }
}
