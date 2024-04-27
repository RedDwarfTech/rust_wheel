use chrono::{Datelike, Duration, Local, NaiveDate, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

/**
 * https://stackoverflow.com/questions/26593387/how-can-i-get-the-current-time-in-milliseconds
 */
pub fn get_current_millisecond() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms as i64;
}

pub fn get_minus_day_millisecond(days: i64) -> i64 {
    let dt = Utc::now() + Duration::days(days);
    return dt.timestamp_millis();
}

pub fn end_of_today() -> i64 {
    Local::now()
        .date_naive()
        .and_hms_milli_opt(23, 59, 59, 999)
        .unwrap()
        .and_utc()
        .timestamp_millis()
}

pub fn start_of_today() -> i64 {
    Local::now()
        .date_naive()
        .and_hms_milli_opt(0, 0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis()
}

pub fn start_of_month() -> i64 {
    let local = Local::now();
    let nd = NaiveDate::from_ymd_opt(local.year(), local.month(), 1).unwrap();
    let start_of_month = nd
        .and_hms_milli_opt(0, 0, 0, 000)
        .unwrap()
        .and_utc()
        .timestamp_millis();
    return start_of_month;
}

pub fn end_of_month() -> i64 {
    let local = Local::now();
    let nd = last_day_of_month(local.year(), local.month());
    let end_of_month = nd.and_hms_milli_opt(23, 59, 59, 999)
    .unwrap()
    .and_utc()
    .timestamp_millis();
    return end_of_month;
}

pub fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    return NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt().unwrap();
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days()
}