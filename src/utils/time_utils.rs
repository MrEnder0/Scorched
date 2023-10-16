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

    //Make use of time format and split str
    match time_format {
        TimeFormat::DateTime => Utc
            .timestamp_opt(since_the_epoch.as_secs() as i64, 0)
            .unwrap()
            .with_timezone(&Utc)
            .to_rfc3339(),
        TimeFormat::Date => Utc
            .timestamp_opt(since_the_epoch.as_secs() as i64, 0)
            .unwrap()
            .with_timezone(&Utc)
            .to_rfc3339()
            .split('T')
            .collect::<Vec<&str>>()[0]
            .to_string(),
        TimeFormat::Time => Utc
            .timestamp_opt(since_the_epoch.as_secs() as i64, 0)
            .unwrap()
            .with_timezone(&Utc)
            .to_rfc3339()
            .split('T')
            .collect::<Vec<&str>>()[1]
            .to_string(),
    }
}
