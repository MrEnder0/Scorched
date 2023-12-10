mod utils;

use owo_colors::{colors::css::*, OwoColorize};
use std::{env, fmt::Debug, fs::OpenOptions, io::prelude::*};

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

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn set_logging_path(mut path: String) {
    if path[path.len() - 1..] != *"/" {
        path.push('/');
    }

    env::set_var("SCORCHED_LOG_PATH", path);
}

/// Logs the given data to the console with the error type and then to a file
fn log_this(data: LogData) {
    // Creates logs folder if it doesn't exist
    if !std::path::Path::new("logs").exists() {
        std::fs::create_dir("logs").unwrap();
    }

    let file = OpenOptions::new().append(true).create(true).open(format!(
        "{}{}.log",
        env::var("SCORCHED_LOG_PATH").unwrap_or_else(|_| "logs/".to_string()),
        utils::time_utils::get_formatted_time(utils::time_utils::TimeFormat::Date)
    ));

    match data.importance {
        LogImportance::Error => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [ERROR] {}\n",
                        utils::time_utils::get_formatted_time(
                            utils::time_utils::TimeFormat::DateTime
                        ),
                        data.message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                utils::time_utils::get_formatted_time(utils::time_utils::TimeFormat::Time),
                "[ERROR]".fg::<Black>().bg::<Red>(),
                data.message
            );
        }
        LogImportance::Warning => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [WARNING] {}\n",
                        utils::time_utils::get_formatted_time(
                            utils::time_utils::TimeFormat::DateTime
                        ),
                        data.message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                utils::time_utils::get_formatted_time(utils::time_utils::TimeFormat::Time),
                "[WARNING]".fg::<Black>().bg::<Yellow>(),
                data.message
            );
        }
        LogImportance::Info => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [INFO] {}\n",
                        utils::time_utils::get_formatted_time(
                            utils::time_utils::TimeFormat::DateTime
                        ),
                        data.message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                utils::time_utils::get_formatted_time(utils::time_utils::TimeFormat::Time),
                "[INFO]".fg::<Black>().bg::<LightGray>(),
                data.message
            );
        }
        // Mainly unused, but still available
        LogImportance::Debug => {
            file.unwrap()
                .write_all(
                    format!(
                        "{} [DEBUG] {}\n",
                        utils::time_utils::get_formatted_time(
                            utils::time_utils::TimeFormat::DateTime
                        ),
                        data.message
                    )
                    .as_bytes(),
                )
                .unwrap();
            println!(
                "{} {} {}",
                utils::time_utils::get_formatted_time(utils::time_utils::TimeFormat::Time),
                "[DEBUG]".fg::<Black>().bg::<Magenta>(),
                data.message
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
