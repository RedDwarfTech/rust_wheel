use super::mobc_error::MobcError;
use crate::config::cache::mobc_error::Error;
use crate::config::cache::mobc_error::MobcError::{RedisCMDError, RedisTypeError};
use log::error;
use redis::{Commands, Connection, FromRedisValue, RedisError, Value};
use std::env;

pub fn get_con() -> Connection {
    let config_redis_string: String = env::var("REDIS_URL").expect("redis url missing");
    let redis_client =
        redis::Client::open(config_redis_string.as_str()).expect("get redis client failed");
    let con = redis_client
        .get_connection()
        .expect("get redis connection failed");
    return con;
}

pub fn get_redis_conn(conn_url: &str) -> Connection {
    let redis_client = redis::Client::open(conn_url).expect("get redis client by url failed");
    let con = redis_client
        .get_connection()
        .expect("get redis connection by url failed");
    return con;
}

pub fn set_value(key: &str, value: &str, ttl_seconds: usize) -> Result<String, MobcError> {
    let mut redis_conn = get_con();
    let set_result: Result<String, MobcError> = redis_conn
        .set_ex(key, value, ttl_seconds)
        .map_err(RedisCMDError);
    return set_result;
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

pub fn get_str_default(key: &str) -> Result<Option<String>, Error> {
    let mut redis_conn = get_con();
    let value = redis_conn.get(key).map_err(RedisCMDError)?;
    if Value::Nil == value {
        return Ok(None);
    }
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}

pub async fn get_str(con: &mut Connection, key: &str) -> Result<Option<String>, Error> {
    let value = con.get(key).map_err(RedisCMDError)?;
    if Value::Nil == value {
        return Ok(None);
    }
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}

pub fn del_redis_key(key: &str) -> Result<(), Error> {
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

pub fn push_data_to_stream(stream_key: &str, params: &[(&str, &str)]) {
    let mut connection = get_con();
    let result = connection.xadd::<&str, &str, &str, &str, String>(stream_key, "*", params);
    match result {
        Ok(_) => {}
        Err(e) => {
            error!("Couldn't send to redis stream,{}", e);
        }
    }
}

pub fn push_to_stream(stream_key: &str, params: &[(&str, &str)]) -> Result<String, RedisError> {
    let mut connection = get_con();
    connection.xadd::<&str, &str, &str, &str, String>(stream_key, "*", params)
}

pub fn push_to_group_stream(
    stream_key: &str,
    group: &str,
    params: &[(&str, &str)],
) -> Result<String, RedisError> {
    let mut connection = get_con();
    let g_result =
        connection.xgroup_create_mkstream::<&str, &str, &str, String>(stream_key, group, "$");
    if let Err(e) = g_result.as_ref() {
        error!(
            "create group failed,{},stream key: {}, group: {}",
            e, stream_key, group
        );
        return g_result;
    }
    connection.xadd::<&str, &str, &str, &str, String>(stream_key, "*", params)
}

pub fn delete_stream_element(stream_key: &str, msg_id: String) -> Result<usize, RedisError> {
    let mut connection = get_con();
    let del_result: Result<usize, RedisError> = connection.xdel(stream_key, &[msg_id]);
    return del_result;
}
