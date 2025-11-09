//! Computer status writer for CSV output
//!
//! This module implements a writer that outputs computer enumeration status as CSV.
//! The CSV format is: `ComputerName,Task,Status,ObjectID`
//!
//! This writer is typically used for debugging and monitoring collection progress.

use super::base_writer::{BaseWriter, BaseWriterState};
use crate::client::Context;
use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// CSV computer status record
///
/// This is a placeholder for CSVComputerStatus from SharpHoundCommonLib.
/// The full implementation will be provided when SharpHoundCommonLib is ported.
///
/// Represents the status of computer enumeration tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvComputerStatus {
    /// Computer name
    pub computer_name: String,

    /// Task being performed (e.g., "LocalAdmins", "Sessions")
    pub task: String,

    /// Status of the task (e.g., "Success", "Failed", "Timeout")
    pub status: String,

    /// Object ID (SID or distinguished name)
    pub object_id: String,
}

impl CsvComputerStatus {
    /// Create a new computer status record
    ///
    /// # Arguments
    ///
    /// * `computer_name` - Name of the computer
    /// * `task` - Task being performed
    /// * `status` - Status of the task
    /// * `object_id` - Object ID (SID or DN)
    pub fn new(
        computer_name: String,
        task: String,
        status: String,
        object_id: String,
    ) -> Self {
        CsvComputerStatus {
            computer_name,
            task,
            status,
            object_id,
        }
    }

    /// Convert to CSV row
    ///
    /// Returns a CSV-formatted string with proper escaping.
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{}",
            Self::escape_csv(&self.computer_name),
            Self::escape_csv(&self.task),
            Self::escape_csv(&self.status),
            Self::escape_csv(&self.object_id)
        )
    }

    /// Escape a field for CSV output
    ///
    /// Fields containing commas, quotes, or newlines are wrapped in quotes.
    /// Internal quotes are doubled.
    fn escape_csv(field: &str) -> String {
        if field.contains(',') || field.contains('"') || field.contains('\n') {
            format!("\"{}\"", field.replace('"', "\"\""))
        } else {
            field.to_string()
        }
    }
}

/// Computer status CSV writer
///
/// Writes computer enumeration status to a CSV file.
/// The output format is: `ComputerName,Task,Status,ObjectID`
///
/// This writer reads from a channel and writes status updates as they arrive.
pub struct CompStatusWriter {
    /// Common writer state
    state: BaseWriterState<CsvComputerStatus>,

    /// Reference to the context
    context: std::sync::Arc<dyn Context>,

    /// Path to the output file
    filename: Option<PathBuf>,

    /// File handle for writing
    file: Option<File>,

    /// Channel for receiving status updates
    channel: mpsc::Receiver<CsvComputerStatus>,
}

impl CompStatusWriter {
    /// Create a new computer status writer
    ///
    /// # Arguments
    ///
    /// * `context` - The collection context
    /// * `channel` - Channel for receiving status updates
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sharphound::writers::CompStatusWriter;
    /// use sharphound::BaseContext;
    /// use std::sync::Arc;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example(context: Arc<BaseContext>) -> anyhow::Result<()> {
    /// let (tx, rx) = mpsc::channel(100);
    /// let writer = CompStatusWriter::new(context, rx);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        context: std::sync::Arc<dyn Context>,
        channel: mpsc::Receiver<CsvComputerStatus>,
    ) -> Self {
        let mut state = BaseWriterState::new("compstatus".to_string());

        // Check if computer status dumping is disabled
        if !context.flags().dump_computer_status {
            state.set_no_op(true);
        }

        CompStatusWriter {
            state,
            context,
            filename: None,
            file: None,
            channel,
        }
    }

    /// Start the writer task
    ///
    /// This method consumes items from the channel and writes them to the CSV file.
    /// It blocks until the channel is closed, then flushes and closes the file.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if writing fails
    pub async fn start_writer(&mut self) -> Result<()> {
        if self.state.no_op {
            // Drain the channel without writing
            while self.channel.recv().await.is_some() {}
            return Ok(());
        }

        while let Some(item) = self.channel.recv().await {
            self.accept_object(item).await?;
        }

        self.flush_writer().await?;
        Ok(())
    }

    /// Write the CSV header
    fn write_header(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.file {
            writeln!(file, "ComputerName,Task,Status,ObjectID")?;
        }
        Ok(())
    }
}

#[async_trait]
impl BaseWriter<CsvComputerStatus> for CompStatusWriter {
    async fn accept_object(&mut self, item: CsvComputerStatus) -> Result<()> {
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
        if let Some(ref mut file) = self.file {
            for item in &self.state.queue {
                writeln!(file, "{}", item.to_csv())?;
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

        // Close file
        if let Some(mut file) = self.file.take() {
            file.flush()?;
        }

        Ok(())
    }

    fn create_file(&mut self) -> Result<()> {
        let filename = self.context.resolve_filename(&self.state.data_type, "csv", true);

        let file = File::create(&filename)
            .with_context(|| format!("Failed to create file: {:?}", filename))?;

        self.filename = Some(filename.clone());
        self.file = Some(file);

        // Write CSV header
        self.write_header()?;

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
    fn test_csv_computer_status_new() {
        let status = CsvComputerStatus::new(
            "DC01".to_string(),
            "LocalAdmins".to_string(),
            "Success".to_string(),
            "S-1-5-21-123".to_string(),
        );

        assert_eq!(status.computer_name, "DC01");
        assert_eq!(status.task, "LocalAdmins");
        assert_eq!(status.status, "Success");
        assert_eq!(status.object_id, "S-1-5-21-123");
    }

    #[test]
    fn test_csv_computer_status_to_csv() {
        let status = CsvComputerStatus::new(
            "DC01".to_string(),
            "Sessions".to_string(),
            "Success".to_string(),
            "S-1-5-21-456".to_string(),
        );

        let csv = status.to_csv();
        assert_eq!(csv, "DC01,Sessions,Success,S-1-5-21-456");
    }

    #[test]
    fn test_csv_escape_simple() {
        assert_eq!(CsvComputerStatus::escape_csv("test"), "test");
        assert_eq!(CsvComputerStatus::escape_csv("DC01"), "DC01");
    }

    #[test]
    fn test_csv_escape_comma() {
        assert_eq!(
            CsvComputerStatus::escape_csv("test,value"),
            "\"test,value\""
        );
    }

    #[test]
    fn test_csv_escape_quotes() {
        assert_eq!(
            CsvComputerStatus::escape_csv("test\"value"),
            "\"test\"\"value\""
        );
    }

    #[test]
    fn test_csv_escape_newline() {
        assert_eq!(
            CsvComputerStatus::escape_csv("test\nvalue"),
            "\"test\nvalue\""
        );
    }

    #[test]
    fn test_csv_escape_complex() {
        let complex = "Computer, \"Name\" with\nnewline";
        let escaped = CsvComputerStatus::escape_csv(complex);
        assert_eq!(escaped, "\"Computer, \"\"Name\"\" with\nnewline\"");
    }

    #[test]
    fn test_csv_computer_status_with_special_chars() {
        let status = CsvComputerStatus::new(
            "DC,01".to_string(),
            "Local\"Admins".to_string(),
            "Success\nFailed".to_string(),
            "S-1-5-21-789".to_string(),
        );

        let csv = status.to_csv();
        assert_eq!(
            csv,
            "\"DC,01\",\"Local\"\"Admins\",\"Success\nFailed\",S-1-5-21-789"
        );
    }

    #[test]
    fn test_csv_computer_status_serialize() {
        let status = CsvComputerStatus::new(
            "DC01".to_string(),
            "LoggedOn".to_string(),
            "Failed".to_string(),
            "S-1-5-21-999".to_string(),
        );

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"computer_name\":\"DC01\""));
        assert!(json.contains("\"task\":\"LoggedOn\""));
    }

    #[test]
    fn test_csv_computer_status_deserialize() {
        let json = r#"{
            "computer_name": "DC02",
            "task": "RDP",
            "status": "Timeout",
            "object_id": "S-1-5-21-111"
        }"#;

        let status: CsvComputerStatus = serde_json::from_str(json).unwrap();

        assert_eq!(status.computer_name, "DC02");
        assert_eq!(status.task, "RDP");
        assert_eq!(status.status, "Timeout");
        assert_eq!(status.object_id, "S-1-5-21-111");
    }
}
