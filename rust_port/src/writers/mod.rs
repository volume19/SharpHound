//! Output writers for BloodHound data
//!
//! This module provides writers for outputting collected data in various formats.
//! Writers batch data in memory and flush to disk periodically for performance.
//!
//! # Writers
//!
//! - `BaseWriter` - Trait defining the writer interface
//! - `JsonDataWriter` - Writes BloodHound JSON format
//! - `CompStatusWriter` - Writes computer status as CSV
//!
//! # Example
//!
//! ```no_run
//! use sharphound::writers::{JsonDataWriter, BaseWriter};
//! use sharphound::BaseContext;
//! use std::sync::Arc;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct User {
//!     name: String,
//!     sid: String,
//! }
//!
//! # async fn example(context: Arc<BaseContext>) -> anyhow::Result<()> {
//! let mut writer = JsonDataWriter::<User>::new(context, "users".to_string());
//!
//! // Write some users
//! writer.accept_object(User {
//!     name: "Administrator".to_string(),
//!     sid: "S-1-5-21-123-456-789-500".to_string(),
//! }).await?;
//!
//! // Flush and close
//! writer.flush_writer().await?;
//! # Ok(())
//! # }
//! ```

pub mod base_writer;
pub mod comp_status_writer;
pub mod json_data_writer;

pub use base_writer::{BaseWriter, BaseWriterState, BATCH_SIZE};
pub use comp_status_writer::{CompStatusWriter, CsvComputerStatus};
pub use json_data_writer::{JsonDataWriter, MetaTag, COLLECTOR_VERSION, DATA_VERSION};
