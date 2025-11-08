//! Logging infrastructure

pub mod basic_logger;

pub use basic_logger::{BasicLogger, LogLevel, init_tracing};
