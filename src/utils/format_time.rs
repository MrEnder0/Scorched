use std::time::{SystemTime, UNIX_EPOCH};

pub enum TimeFormat {
    Ymdhms,
    Ymd,
    //Currently unused
    HMS,
}

pub fn get_formatted_time(format: TimeFormat) -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let seconds_since_epoch = duration_since_epoch.as_secs();

    match format {
        TimeFormat::Ymdhms => {
            let formatted_time = format!(
                //Y-m-d H:M:S
                "{}-{}-{} {}:{}.{}",
                seconds_since_epoch / 31536000 + 1970,
                (seconds_since_epoch % 31556926) / 2629743 + 1,
                (seconds_since_epoch % 2629743) / 86400 + 1,
                (seconds_since_epoch % 86400) / 3600 + 0,
                (seconds_since_epoch % 3600) / 60,
                seconds_since_epoch % 60
            );
            return formatted_time;
        }
        TimeFormat::Ymd => {
            let formatted_time = format!(
                //Y-m-d
                "{}-{}-{}",
                seconds_since_epoch / 31536000 + 1970,
                (seconds_since_epoch % 31556926) / 2629743 + 1,
                (seconds_since_epoch % 2629743) / 86400 + 1,
            );
            return formatted_time;
        }
        TimeFormat::HMS => {
            let formatted_time = format!(
                //H:M:S
                "{}:{}:{}",
                (seconds_since_epoch % 86400) / 3600 + 0,
                (seconds_since_epoch % 3600) / 60,
                seconds_since_epoch % 60
            );
            return formatted_time;
        }
    }
}