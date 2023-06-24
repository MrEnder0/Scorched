use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub enum TimeFormat {
    Ymdhms,
    Ymd,
    //Currently unused
    HMS,
}

pub fn get_formatted_time(format: TimeFormat) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    match format {
        TimeFormat::Ymdhms => {
            let formatted_time = format!(
                "{}",
                Utc.timestamp(since_the_epoch.as_secs() as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
            );
            return formatted_time;
        }
        TimeFormat::Ymd => {
            let formatted_time = format!(
                "{}",
                Utc.timestamp(since_the_epoch.as_secs() as i64, 0)
                    .format("%Y-%m-%d")
            );
            return formatted_time;
        }
        TimeFormat::HMS => {
            let formatted_time = format!(
                "{}",
                Utc.timestamp(since_the_epoch.as_secs() as i64, 0)
                    .format("%H:%M:%S")
            );
            return formatted_time;
        }
    }
}
