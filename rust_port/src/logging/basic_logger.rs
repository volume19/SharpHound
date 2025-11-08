//! Basic console logging implementation
//!
//! This module provides a simple console logger with verbosity filtering,
//! compatible with the original C# BasicLogger behavior.

use chrono::Local;
use std::fmt;
use tracing::Level;

/// Log level matching Microsoft.Extensions.Logging.LogLevel
///
/// Numeric values:
/// - Trace = 0
/// - Debug = 1
/// - Information = 2
/// - Warning = 3
/// - Error = 4
/// - Critical = 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Information = 2,
    Warning = 3,
    Error = 4,
    Critical = 5,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Information => write!(f, "INFORMATION"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Information => Level::INFO,
            LogLevel::Warning => Level::WARN,
            LogLevel::Error | LogLevel::Critical => Level::ERROR,
        }
    }
}

/// Basic console logger with verbosity filtering
///
/// This logger mimics the behavior of the C# BasicLogger:
/// - Filters messages based on verbosity level
/// - Outputs to console with timestamp
/// - Format: {timestamp}|{LEVEL}|{message}
pub struct BasicLogger {
    verbosity: u8,
}

impl BasicLogger {
    /// Create a new BasicLogger with the specified verbosity level
    ///
    /// # Arguments
    /// * `verbosity` - Minimum log level to display (0=Trace, 1=Debug, 2=Information, 3=Warning, 4=Error, 5=Critical)
    ///
    /// # Example
    /// ```
    /// use sharphound::logging::basic_logger::BasicLogger;
    ///
    /// // Show Information level and above (Information, Warning, Error, Critical)
    /// let logger = BasicLogger::new(2);
    /// ```
    pub fn new(verbosity: u8) -> Self {
        Self { verbosity }
    }

    /// Check if a log level is enabled
    pub fn is_enabled(&self, level: LogLevel) -> bool {
        (level as u8) >= self.verbosity
    }

    /// Log a message at the specified level
    pub fn log(&self, level: LogLevel, message: &str) {
        if self.is_enabled(level) {
            self.write_level(level, message, None::<&str>);
        }
    }

    /// Log a message with an error
    pub fn log_with_error(&self, level: LogLevel, message: &str, error: &str) {
        if self.is_enabled(level) {
            self.write_level(level, message, Some(error));
        }
    }

    /// Write a log message to console
    fn write_level(&self, level: LogLevel, message: &str, error: Option<&str>) {
        println!("{}", Self::format_log(level, message, error));
    }

    /// Format a log message
    ///
    /// Format: {ISO8601_timestamp}|{LEVEL}|{message}
    fn format_log(level: LogLevel, message: &str, error: Option<&str>) -> String {
        let time = Local::now();
        let timestamp = time.to_rfc3339();

        match error {
            Some(err) => format!("{}|{}|{}\n{}", timestamp, level, message, err),
            None => format!("{}|{}|{}", timestamp, level, message),
        }
    }

    /// Convenience method: log at Trace level
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }

    /// Convenience method: log at Debug level
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Convenience method: log at Information level
    pub fn information(&self, message: &str) {
        self.log(LogLevel::Information, message);
    }

    /// Convenience method: log at Warning level
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    /// Convenience method: log at Error level
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Convenience method: log at Critical level
    pub fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }
}

/// Initialize tracing subscriber for application-wide logging
///
/// This configures the tracing framework with console output
pub fn init_tracing(verbosity: u8) {
    let level = match verbosity {
        0 => Level::TRACE,
        1 => Level::DEBUG,
        2 => Level::INFO,
        3 => Level::WARN,
        4 | 5 => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Information);
        assert!(LogLevel::Information < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Critical);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Information), "INFORMATION");
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
        assert_eq!(format!("{}", LogLevel::Critical), "CRITICAL");
    }

    #[test]
    fn test_logger_is_enabled() {
        let logger = BasicLogger::new(2); // Information level

        assert!(!logger.is_enabled(LogLevel::Trace));
        assert!(!logger.is_enabled(LogLevel::Debug));
        assert!(logger.is_enabled(LogLevel::Information));
        assert!(logger.is_enabled(LogLevel::Warning));
        assert!(logger.is_enabled(LogLevel::Error));
        assert!(logger.is_enabled(LogLevel::Critical));
    }

    #[test]
    fn test_logger_format() {
        let formatted = BasicLogger::format_log(LogLevel::Information, "Test message", None);
        assert!(formatted.contains("|INFORMATION|Test message"));

        let formatted_with_error = BasicLogger::format_log(
            LogLevel::Error,
            "Error occurred",
            Some("Stack trace here"),
        );
        assert!(formatted_with_error.contains("|ERROR|Error occurred"));
        assert!(formatted_with_error.contains("Stack trace here"));
    }

    #[test]
    fn test_logger_new() {
        let logger = BasicLogger::new(2);
        assert_eq!(logger.verbosity, 2);
    }
}
