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
//! - `context` - Context implementations
//! - `domain` - Domain enumeration types
//! - `logging` - Logging infrastructure
//! - `util` - Utility functions and extension traits

pub mod cli;
pub mod client;
pub mod context;
pub mod domain;
pub mod logging;
pub mod util;

// Re-export commonly used types
pub use cli::Options;
pub use client::{CollectionMethodOptions, Context, Flags, LdapConfig, Links};
pub use context::BaseContext;
pub use domain::EnumerationDomain;
pub use logging::{BasicLogger, LogLevel};
pub use util::{AsyncStreamExt, CollectionMethodExt, DnsNameResolver, HashMapExt};
