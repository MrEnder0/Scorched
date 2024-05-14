#[test]
fn test_log_this() {
    use crate::{log_this, LogData, LogImportance};

    log_this(LogData {
        importance: LogImportance::Error,
        message: "Test error".to_string(),
    });

    log_this(LogData {
        importance: LogImportance::Warning,
        message: "Test warning".to_string(),
    });

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Test info".to_string(),
    });

    log_this(LogData {
        importance: LogImportance::Debug,
        message: "Test debug".to_string(),
    });
}

#[test]
fn test_logf() {
    use crate::{logf, LogData, LogImportance};

    logf!(Error, "Test error");

    logf!(Warning, "Test warning");

    logf!(Info, "Test info");

    logf!(Debug, "Test debug");
}

#[test]
fn test_log_path() {
    use crate::*;

    set_logging_path("logs/test");

    log_this(LogData {
        importance: LogImportance::Error,
        message: "Test error".to_string(),
    });

    assert_eq!(*LOG_PATH.get().unwrap(), "logs/test/");
}