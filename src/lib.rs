mod macros;
mod utils;

use std::{env, fmt::Debug, fs::OpenOptions, io::prelude::*};

use utils::{importance_tags::*, time_utils};

pub enum LogImportance {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct LogData {
    pub importance: LogImportance,
    pub message: String,
}

/// Scorched version, has no internal use
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Changes environment variable for logging path
pub fn set_logging_path(path: &str) {
    if path[path.len() - 1..] != *"/" {
        let mut path = path.to_string();
        path.push('/');
    }

    env::set_var("SCORCHED_LOG_PATH", path);
}

pub fn set_log_prefix(prefix: &str) {
    env::set_var("SCORCHED_LOG_PREFIX", prefix);
}

/// Logs the given data to the console with the error type and then to a file
pub fn log_this(data: LogData) {
    // Creates logs folder if it doesn't exist
    if !std::path::Path::new(
        env::var("SCORCHED_LOG_PATH")
            .unwrap_or_else(|_| "logs/".to_string())
            .as_str(),
    )
    .exists()
    {
        std::fs::create_dir_all(
            env::var("SCORCHED_LOG_PATH").unwrap_or_else(|_| "logs/".to_string()),
        )
        .log_expect(LogImportance::Error, "Failed to create logs folder");
    }

    let file = OpenOptions::new().append(true).create(true).open(format!(
        "{}{}.log",
        env::var("SCORCHED_LOG_PATH").unwrap_or_else(|_| "logs/".to_string()),
        time_utils::get_formatted_time(time_utils::TimeFormat::Date)
    ));

    let message = {
        match env::var("SCORCHED_LOG_PREFIX") {
            Ok(val) => format!("{} {}", val, data.message),
            Err(_) => data.message,
        }
    };

    match data.importance {
        LogImportance::Error => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [ERROR] {}\n",
                        time_utils::get_formatted_time(time_utils::TimeFormat::DateTime),
                        message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                time_utils::get_formatted_time(time_utils::TimeFormat::Time),
                error_tag(),
                message
            );
        }
        LogImportance::Warning => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [WARNING] {}\n",
                        time_utils::get_formatted_time(time_utils::TimeFormat::DateTime),
                        message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                time_utils::get_formatted_time(time_utils::TimeFormat::Time),
                warning_tag(),
                message
            );
        }
        LogImportance::Info => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [INFO] {}\n",
                        time_utils::get_formatted_time(time_utils::TimeFormat::DateTime),
                        message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                time_utils::get_formatted_time(time_utils::TimeFormat::Time),
                info_tag(),
                message
            );
        }
        // Mainly unused, but still available
        LogImportance::Debug => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [DEBUG] {}\n",
                        time_utils::get_formatted_time(time_utils::TimeFormat::DateTime),
                        message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                time_utils::get_formatted_time(time_utils::TimeFormat::Time),
                debug_tag(),
                message
            );
        }
    }
}

pub trait LogExpect<T, E: Debug> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T, E> for Result<T, E> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                log_this(LogData {
                    importance,
                    message: msg.to_string(),
                });

                panic!("{}: {:?}", msg, err);
            }
        }
    }
}

impl<T> LogExpect<T, ()> for Option<T> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                log_this(LogData {
                    importance,
                    message: msg.to_string(),
                });

                panic!("{}", msg);
            }
        }
    }
}
