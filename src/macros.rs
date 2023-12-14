#[macro_export]
macro_rules! logf {
    ($importance:ident, $($arg:tt)*) => {
        crate::log_this(LogData {
            importance: LogImportance::$importance,
            message: format!($($arg)*).to_string(),
        });
    };
}
