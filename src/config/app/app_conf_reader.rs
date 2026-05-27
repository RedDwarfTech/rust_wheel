use config::{Config, File};
use log::error;

pub fn get_app_config(key: &str) -> String {
    let settings = Config::builder()
        .add_source(File::with_name("settings"))
        .build();
    let settings = match settings {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("merge config error: {}, key:{}", err, key);
            return "".to_string();
        }
    };
    let read_result = settings.get::<String>(key);
    if let Err(e) = read_result {
        error!("read config error: {}, key: {}", e, key);
        return "".to_string();
    }
    return read_result.unwrap();
}

pub fn get_i64_app_config(key: &str) -> Option<i64> {
    let settings = Config::builder()
        .add_source(File::with_name("settings"))
        .build();
    let settings = match settings {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("merge config error: {}, key:{}", err, key);
            return None;
        }
    };
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
