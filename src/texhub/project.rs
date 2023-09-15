use chrono::{NaiveDateTime, Datelike};
use crate::common::util::rd_file_util::join_paths;

pub fn get_proj_path(base_dir: &String, created_time: i64) -> String {
    let datetime = NaiveDateTime::from_timestamp_opt(created_time, 0);
    let year = datetime.unwrap().year();
    let month = datetime.unwrap().month();
    return join_paths(&[base_dir.as_str(), year.to_string().as_str(), month.to_string().as_str()]);
}