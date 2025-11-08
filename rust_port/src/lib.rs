//! SharpHound - Active Directory Reconnaissance Tool (Rust Port)
//!
//! This is a Rust port of the SharpHound C# tool, which is used for
//! authorized Active Directory enumeration and security assessment.
//!
//! **IMPORTANT**: This tool is designed for authorized security testing only:
//! - Penetration testing engagements
//! - Security audits and assessments
//! - Red team operations (with authorization)
//! - Educational and research purposes
//!
//! # Modules
//!
//! - `client` - Core types, enums, and configuration flags
//! - `cli` - Command-line argument parsing
//! - `domain` - Domain enumeration types
//! - `logging` - Logging infrastructure

pub mod cli;
pub mod client;
pub mod domain;
pub mod logging;

// Re-export commonly used types
pub use cli::Options;
pub use client::{CollectionMethodOptions, Flags};
pub use domain::EnumerationDomain;
pub use logging::{BasicLogger, LogLevel};
