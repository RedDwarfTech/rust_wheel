use std::collections::HashMap;
use log::LevelFilter;
use env_logger::Builder;
use std::io::Write;
use chrono::Local;

pub fn initial_config() {
    initial_log_config();
    initial_file_config();
}

#[allow(dead_code)]
pub fn get_config(key: &str) -> String {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    let hash_config: HashMap<String, String> = settings.try_deserialize().unwrap();
    let conn = hash_config.get(key).unwrap();
    let std =String::from(conn);
    return std;
}

pub fn initial_log_config(){
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

pub fn initial_file_config(){
    let settings = config::Config::builder()
        .add_source(config::File::with_name("settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    let hash_config: HashMap<String, String> = settings.try_deserialize().unwrap();
    println!("{:?}", hash_config);
}
