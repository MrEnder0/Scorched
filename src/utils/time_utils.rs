use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub enum TimeFormat {
    DateTime,
    Date,
    Time,
}

pub fn get_formatted_time(time_format: TimeFormat) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let time = Utc
        .timestamp_opt(since_the_epoch.as_secs() as i64, 0)
        .unwrap()
        .with_timezone(&Utc)
        .to_rfc3339();

    // Splits the time into the needed format
    match time_format {
        TimeFormat::DateTime => time,
        TimeFormat::Date => time.split('T').collect::<Vec<&str>>()[0].to_string(),
        TimeFormat::Time => time.split('T').collect::<Vec<&str>>()[1].to_string(),
    }
}
