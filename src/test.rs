use crate::{log_this, logf, LogData, LogImportance};

#[test]
fn test_log_this() {
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
    logf!(Error, "Test error");

    logf!(Warning, "Test warning");

    logf!(Info, "Test info");

    logf!(Debug, "Test debug");
}
