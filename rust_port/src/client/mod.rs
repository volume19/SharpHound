//! Client module containing core types and interfaces
//!
//! This module defines the fundamental types used throughout SharpHound,
//! including enums for collection methods, configuration flags, and
//! the context trait that manages collection state.

pub mod context;
pub mod enums;
pub mod flags;
pub mod links;

pub use context::{Context, ContextUtils, FileExistsError};
pub use enums::CollectionMethodOptions;
pub use flags::Flags;
pub use links::{AuthType, LdapConfig, Links};
