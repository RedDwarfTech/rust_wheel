use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Duration, Utc};

pub fn get_current_millisecond() -> i64{
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms as i64;
}

pub fn get_minus_day_millisecond(days: i64) -> i64 {
    let dt = Utc::now() + Duration::days(days);
    return dt.timestamp_millis();
}



