//! Data producers for Active Directory enumeration
//!
//! This module provides producers that collect Active Directory data from various sources.
//! Producers retrieve directory objects and write them to channels for downstream processing.
//!
//! # Producers
//!
//! - `BaseProducer` - Trait defining the producer interface
//! - `LdapProducer` - Main LDAP-based producer (standard collection)
//! - `ComputerFileProducer` - Reads computers from a text file
//! - `StealthProducer` - Stealth collection mode (targeted computers only)
//!
//! # Collection Flow
//!
//! ```text
//! Producer → Channel<DirectoryObject> → Consumer → Channel<OutputBase> → Writer
//! ```
//!
//! 1. **Producer**: Queries LDAP or files for directory objects
//! 2. **Channel**: Async message passing via tokio::sync::mpsc
//! 3. **Consumer**: Processes objects (ACLs, sessions, etc.) [Phase 6]
//! 4. **Output**: Formatted data (users, computers, groups)
//! 5. **Writer**: Writes to JSON files [Phase 3 - complete]
//!
//! # LDAP Integration Status
//!
//! **Current Status**: Placeholder implementations
//!
//! Producers depend on LDAP functionality that will be implemented in Phase 5:
//! - LDAP connection and authentication
//! - Paged queries
//! - DN/SID resolution
//! - Security descriptor retrieval
//!
//! **Placeholder Types**: See `common_lib_placeholder` module for types that will be
//! replaced when SharpHoundCommonLib is ported (Phase 8-9).
//!
//! # Example (when LDAP is implemented)
//!
//! ```no_run
//! use sharphound::producers::{LdapProducer, BaseProducer, BaseProducerState};
//! use sharphound::BaseContext;
//! use std::sync::Arc;
//! use tokio::sync::mpsc;
//!
//! # async fn example(context: Arc<BaseContext>) -> anyhow::Result<()> {
//! let (dir_tx, dir_rx) = mpsc::channel(1000);
//! let (out_tx, out_rx) = mpsc::channel(1000);
//! let (status_tx, status_rx) = mpsc::channel(100);
//!
//! let state = BaseProducerState::new(
//!     context.clone(),
//!     dir_tx,
//!     out_tx,
//!     status_tx,
//! );
//!
//! let mut producer = LdapProducer::new(state);
//!
//! // Produce objects from default naming context
//! producer.produce().await?;
//!
//! // Produce objects from configuration naming context
//! producer.produce_config_nc().await?;
//! # Ok(())
//! # }
//! ```

pub mod base_producer;
pub mod common_lib_placeholder;
pub mod computer_file_producer;
pub mod ldap_producer;
pub mod stealth_producer;

pub use base_producer::{BaseProducer, BaseProducerState};
pub use common_lib_placeholder::{
    ComputerStatus, DirectoryObject, GeneratedLdapParameters, LdapFilter, LdapQueryParameters,
    LdapResult, NamingContext, OutputBase, SearchScope,
};
pub use computer_file_producer::ComputerFileProducer;
pub use ldap_producer::LdapProducer;
pub use stealth_producer::StealthProducer;
