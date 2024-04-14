use chrono::{DateTime, Datelike};
use crate::common::util::rd_file_util::join_paths;

pub fn get_proj_path(base_dir: &String, created_time: i64) -> String {
    let created_time_in_seconds = created_time / 1000;
    let datetime = DateTime::from_timestamp(created_time_in_seconds, 0);
    let year = datetime.unwrap().year();
    let month = datetime.unwrap().month();
    return join_paths(&[base_dir.as_str(), year.to_string().as_str(), month.to_string().as_str()]);
}

pub fn get_proj_relative_path(proj_id: &String, created_time: i64) -> String {
    let created_time_in_seconds = created_time / 1000;
    let datetime = DateTime::from_timestamp(created_time_in_seconds, 0);
    let year = datetime.unwrap().year();
    let month = datetime.unwrap().month();
    return join_paths(&[year.to_string().as_str(), month.to_string().as_str(), proj_id]);
}