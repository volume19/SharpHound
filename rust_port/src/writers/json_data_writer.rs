//! JSON data writer for BloodHound output files
//!
//! This module implements a writer that outputs data in the BloodHound JSON format.
//! The output structure is:
//! ```json
//! {
//!   "data": [
//!     { ... objects ... }
//!   ],
//!   "meta": {
//!     "count": 100,
//!     "type": "users",
//!     "version": 6,
//!     "methods": 12345,
//!     "collector_version": "2.8.0"
//!   }
//! }
//! ```

use super::base_writer::{BaseWriter, BaseWriterState};
use crate::client::Context;
use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// BloodHound data format version
///
/// This constant defines the version of the BloodHound data format being used.
/// BloodHound uses this to ensure compatibility when loading data files.
pub const DATA_VERSION: u32 = 6;

/// SharpHound collector version
///
/// This matches the version of the tool generating the data.
pub const COLLECTOR_VERSION: &str = "2.8.0";

/// Metadata tag for JSON output files
///
/// This structure is written at the end of each JSON file to provide
/// metadata about the collection run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTag {
    /// Number of objects in the data array
    #[serde(rename = "count")]
    pub count: usize,

    /// Type of data (e.g., "users", "computers", "groups")
    #[serde(rename = "type")]
    pub data_type: String,

    /// BloodHound data format version
    #[serde(rename = "version")]
    pub version: u32,

    /// Bitmask of collection methods used
    ///
    /// This is a placeholder - in the full implementation, this will be
    /// derived from CollectionMethod flags from SharpHoundCommonLib.
    #[serde(rename = "methods")]
    pub collection_methods: u64,

    /// Version of the SharpHound collector
    #[serde(rename = "collectorVersion")]
    pub collector_version: String,
}

impl MetaTag {
    /// Create a new metadata tag
    ///
    /// # Arguments
    ///
    /// * `count` - Number of objects written
    /// * `data_type` - Type of data (e.g., "users", "computers")
    /// * `collection_methods` - Bitmask of collection methods used
    pub fn new(count: usize, data_type: String, collection_methods: u64) -> Self {
        MetaTag {
            count,
            data_type,
            version: DATA_VERSION,
            collection_methods,
            collector_version: COLLECTOR_VERSION.to_string(),
        }
    }
}

/// JSON data writer
///
/// Writes BloodHound data as JSON files with the format:
/// `{ "data": [...], "meta": {...} }`
///
/// # Type Parameters
///
/// * `T` - The type of objects being written (must be Serialize + Send + Sync)
pub struct JsonDataWriter<T: Serialize + Send + Sync> {
    /// Common writer state
    state: BaseWriterState<T>,

    /// Reference to the context
    context: std::sync::Arc<dyn Context>,

    /// Path to the output file
    filename: Option<PathBuf>,

    /// File handle for writing
    file: Option<File>,

    /// Whether to use pretty-printed JSON
    pretty_print: bool,
}

impl<T: Serialize + Send + Sync> JsonDataWriter<T> {
    /// Create a new JSON data writer
    ///
    /// # Arguments
    ///
    /// * `context` - The collection context
    /// * `data_type` - Type of data being written (e.g., "users", "computers")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sharphound::writers::JsonDataWriter;
    /// use sharphound::BaseContext;
    /// use std::sync::Arc;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     sid: String,
    /// }
    ///
    /// # async fn example(context: Arc<BaseContext>) -> anyhow::Result<()> {
    /// let writer = JsonDataWriter::<User>::new(context, "users".to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(context: std::sync::Arc<dyn Context>, data_type: String) -> Self {
        let mut state = BaseWriterState::new(data_type);

        // Check if output is disabled
        if context.flags().no_output {
            state.set_no_op(true);
        }

        let pretty_print = context.flags().pretty_print;

        JsonDataWriter {
            state,
            context,
            filename: None,
            file: None,
            pretty_print,
        }
    }

    /// Get the filename being written to
    ///
    /// Returns None if no file has been created yet.
    pub fn get_filename(&self) -> Option<&PathBuf> {
        self.filename.as_ref()
    }

    /// Write the JSON preamble
    ///
    /// Writes: `{"data":[`
    fn write_preamble(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.file {
            if self.pretty_print {
                writeln!(file, "{{")?;
                writeln!(file, "  \"data\": [")?;
            } else {
                write!(file, "{{\"data\":[")?;
            }
        }
        Ok(())
    }

    /// Write the JSON epilogue with metadata
    ///
    /// Writes: `],"meta":{...}}`
    fn write_epilogue(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.file {
            // TODO: Get actual collection methods from context
            // For now, use a placeholder value
            let collection_methods = 0u64;

            let meta = MetaTag::new(
                self.state.count,
                self.state.data_type.clone(),
                collection_methods,
            );

            if self.pretty_print {
                writeln!(file)?;
                writeln!(file, "  ],")?;
                writeln!(file, "  \"meta\": {{")?;
                writeln!(file, "    \"count\": {},", meta.count)?;
                writeln!(file, "    \"type\": \"{}\",", meta.data_type)?;
                writeln!(file, "    \"version\": {},", meta.version)?;
                writeln!(file, "    \"methods\": {},", meta.collection_methods)?;
                writeln!(file, "    \"collectorVersion\": \"{}\"", meta.collector_version)?;
                writeln!(file, "  }}")?;
                writeln!(file, "}}")?;
            } else {
                write!(file, "],\"meta\":")?;
                let meta_json = serde_json::to_string(&meta)?;
                write!(file, "{}}}", meta_json)?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<T: Serialize + Send + Sync + 'static> BaseWriter<T> for JsonDataWriter<T> {
    async fn accept_object(&mut self, item: T) -> Result<()> {
        if self.state.no_op {
            return Ok(());
        }

        if !self.state.file_created {
            self.create_file()?;
            self.state.mark_file_created();
        }

        self.state.enqueue(item);

        if self.state.should_flush() {
            self.write_data().await?;
            self.state.clear_queue();
        }

        Ok(())
    }

    async fn write_data(&mut self) -> Result<()> {
        // Determine if this is the first write
        let is_first_batch = self.state.count == self.state.queue.len();

        // Serialize all items to JSON first to avoid borrow checker issues
        let mut json_items = Vec::new();
        for item in &self.state.queue {
            let json = if self.pretty_print {
                serde_json::to_string_pretty(item)?
            } else {
                serde_json::to_string(item)?
            };
            json_items.push(json);
        }

        // Now write the JSON strings
        if let Some(ref mut file) = self.file {
            for (idx, json) in json_items.iter().enumerate() {
                let is_first = is_first_batch && idx == 0;

                if !is_first {
                    if self.pretty_print {
                        writeln!(file, ",")?;
                    } else {
                        write!(file, ",")?;
                    }
                }

                if self.pretty_print {
                    // Indent the JSON
                    for line in json.lines() {
                        writeln!(file, "    {}", line)?;
                    }
                } else {
                    write!(file, "{}", json)?;
                }
            }

            file.flush()?;
        }

        Ok(())
    }

    async fn flush_writer(&mut self) -> Result<()> {
        if !self.state.file_created {
            return Ok(());
        }

        // Write any remaining queued items
        if !self.state.queue.is_empty() {
            self.write_data().await?;
        }

        // Write epilogue and close
        self.write_epilogue().context("Failed to write epilogue")?;

        if let Some(mut file) = self.file.take() {
            file.flush()?;
        }

        Ok(())
    }

    fn create_file(&mut self) -> Result<()> {
        let filename = self.context.resolve_filename(&self.state.data_type, "json", true);

        // Check if file already exists
        if filename.exists() {
            anyhow::bail!("File {:?} already exists. This should never happen!", filename);
        }

        let file = File::create(&filename)
            .with_context(|| format!("Failed to create file: {:?}", filename))?;

        self.filename = Some(filename.clone());
        self.file = Some(file);

        // Write preamble
        self.write_preamble()?;

        Ok(())
    }

    fn is_no_op(&self) -> bool {
        self.state.no_op
    }

    fn count(&self) -> usize {
        self.state.count
    }

    fn is_file_created(&self) -> bool {
        self.state.file_created
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_tag_new() {
        let meta = MetaTag::new(100, "users".to_string(), 0x1234);

        assert_eq!(meta.count, 100);
        assert_eq!(meta.data_type, "users");
        assert_eq!(meta.version, DATA_VERSION);
        assert_eq!(meta.collection_methods, 0x1234);
        assert_eq!(meta.collector_version, COLLECTOR_VERSION);
    }

    #[test]
    fn test_meta_tag_serialize() {
        let meta = MetaTag::new(50, "computers".to_string(), 0xABCD);
        let json = serde_json::to_string(&meta).unwrap();

        assert!(json.contains("\"count\":50"));
        assert!(json.contains("\"type\":\"computers\""));
        assert!(json.contains(&format!("\"version\":{}", DATA_VERSION)));
        assert!(json.contains("\"methods\":43981")); // 0xABCD = 43981
        assert!(json.contains(&format!("\"collectorVersion\":\"{}\"", COLLECTOR_VERSION)));
    }

    #[test]
    fn test_meta_tag_deserialize() {
        let json = r#"{
            "count": 75,
            "type": "groups",
            "version": 6,
            "methods": 255,
            "collectorVersion": "2.8.0"
        }"#;

        let meta: MetaTag = serde_json::from_str(json).unwrap();

        assert_eq!(meta.count, 75);
        assert_eq!(meta.data_type, "groups");
        assert_eq!(meta.version, 6);
        assert_eq!(meta.collection_methods, 255);
        assert_eq!(meta.collector_version, "2.8.0");
    }

    #[test]
    fn test_meta_tag_roundtrip() {
        let original = MetaTag::new(200, "domains".to_string(), 0x123456);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: MetaTag = serde_json::from_str(&json).unwrap();

        assert_eq!(original.count, deserialized.count);
        assert_eq!(original.data_type, deserialized.data_type);
        assert_eq!(original.version, deserialized.version);
        assert_eq!(original.collection_methods, deserialized.collection_methods);
        assert_eq!(original.collector_version, deserialized.collector_version);
    }

    #[test]
    fn test_data_version_constant() {
        // Ensure data version matches C# implementation
        assert_eq!(DATA_VERSION, 6);
    }

    #[test]
    fn test_collector_version_constant() {
        // Ensure collector version is set
        assert!(!COLLECTOR_VERSION.is_empty());
        assert_eq!(COLLECTOR_VERSION, "2.8.0");
    }
}
