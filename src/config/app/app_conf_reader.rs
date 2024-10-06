use config::{Config, File};
use log::error;

pub fn get_app_config(key: &str) -> String {
    let mut settings = Config::default();
    let merge_result = settings.merge(File::with_name("settings"));
    if let Err(err) = merge_result {
        error!("merge config error: {}, key:{}", err, key);
        return "".to_string();
    }
    let read_result = settings.get::<String>(key);
    if let Err(e) = read_result {
        error!("read config error: {}, key: {}", e, key);
        return "".to_string();
    }
    return read_result.unwrap();
}

pub fn get_i64_app_config(key: &str) -> Option<i64> {
    let mut settings = Config::default();
    let merge_result = settings.merge(File::with_name("settings"));
    if let Err(err) = merge_result {
        error!("merge config error: {}, key:{}", err, key);
        return None;
    }
    let read_result = settings.get::<String>(key);
    if let Err(e) = read_result {
        error!("read config error: {}, key: {}", e, key);
        return None;
    }
    let parse_result = read_result.unwrap().parse::<i64>();
    if let Err(e) = parse_result.as_ref() {
        error!("parse config failed,{}", e);
    }
    return Some(parse_result.unwrap());
}
