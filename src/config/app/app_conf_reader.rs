use config::{Config, File};

pub fn get_app_config(key: &str) -> String{
    let mut settings = Config::default();
    settings.merge(File::with_name("settings")).unwrap();
    let read_result = settings.get::<String>(key).unwrap();
    return read_result;
}