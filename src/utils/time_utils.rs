use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_formatted_time() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    Utc.timestamp_opt(since_the_epoch.as_secs() as i64, 0)
        .unwrap()
        .with_timezone(&Utc)
        .to_rfc3339()
}
