//! Computer file producer
//!
//! This producer reads computer names from a text file and resolves them to LDAP objects.
//! This is used with the `--computerfile` option to enumerate a specific list of computers.

use super::base_producer::{BaseProducer, BaseProducerState};
use super::common_lib_placeholder::{ComputerStatus, DirectoryObject};
use crate::writers::CsvComputerStatus;
use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

/// Computer file producer
///
/// Reads computer names from a text file (one per line) and resolves them to
/// LDAP objects. Computers can be specified as:
/// - Hostnames (e.g., "DC01", "SERVER01.example.com")
/// - SIDs (e.g., "S-1-5-21-...")
///
/// Each computer is resolved via LDAP and written to the channel for processing.
pub struct ComputerFileProducer {
    /// Common producer state
    state: BaseProducerState,

    /// Path to the computer file
    computer_file: PathBuf,
}

impl ComputerFileProducer {
    /// Create a new computer file producer
    ///
    /// # Arguments
    ///
    /// * `state` - Producer state (context + channels)
    /// * `computer_file` - Path to the file containing computer names
    pub fn new(state: BaseProducerState, computer_file: PathBuf) -> Self {
        ComputerFileProducer {
            state,
            computer_file,
        }
    }
}

#[async_trait]
impl BaseProducer for ComputerFileProducer {
    async fn produce(&mut self) -> Result<()> {
        let ldap_data = self.create_default_nc_data();

        // TODO: Implement CollectAllProperties check
        // if self.state.context.flags().collect_all_properties {
        //     ldap_data.attributes = vec!["*".to_string()];
        // }

        // Get domain name
        let domain_name = if let Some(domain) = self.state.context.domain_name() {
            domain.to_string()
        } else {
            // TODO: Implement GetDomain() from LDAP utils
            anyhow::bail!("No domain name specified for computer file producer and unable to resolve a domain name");
        };

        // Open the computer file
        let file = File::open(&self.computer_file)
            .await
            .with_context(|| format!("Failed to open computer file: {:?}", self.computer_file))?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Process each line
        while let Some(line) = lines.next_line().await? {
            // Check for cancellation
            if self.state.context.is_cancelled() {
                break;
            }

            let computer = line.trim();
            if computer.is_empty() {
                continue;
            }

            // Determine if it's a SID or hostname
            let sid = if computer.starts_with("S-1-5-21") {
                // Already a SID
                computer.to_string()
            } else {
                // TODO: Implement ResolveHostToSid() from LDAP utils
                // For now, log a placeholder message
                tracing::info!("Need to resolve host {} to SID", computer);

                // Write status
                let status = CsvComputerStatus::new(
                    computer.to_string(),
                    "ComputerFileProducer - Produce".to_string(),
                    "Pending".to_string(), // TODO: Would be "Success" after resolution
                    "S-1-5-21-placeholder".to_string(),
                );

                self.state
                    .comp_status_channel
                    .send(status)
                    .await
                    .ok();

                // Placeholder SID until LDAP is implemented
                "S-1-5-21-placeholder".to_string()
            };

            // TODO: Query LDAP to get the full directory object
            // For now, create a placeholder object
            let directory_object = DirectoryObject::new(format!("CN={},{}", computer, domain_name));

            // Write to channel
            self.state
                .channel
                .send(directory_object)
                .await
                .with_context(|| "Failed to write to channel")?;
        }

        Ok(())
    }

    async fn produce_config_nc(&mut self) -> Result<()> {
        // Configuration NC doesn't make sense for computer file input
        // This is intentionally a no-op
        Ok(())
    }

    fn context(&self) -> &Arc<dyn crate::client::Context> {
        &self.state.context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BaseContext, Flags};
    use std::io::Write;
    use tempfile::NamedTempFile;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_computer_file_producer_create() {
        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let computer_file = PathBuf::from("/tmp/computers.txt");

        let producer = ComputerFileProducer::new(state, computer_file);
        assert_eq!(producer.computer_file, PathBuf::from("/tmp/computers.txt"));
    }

    #[tokio::test]
    async fn test_computer_file_producer_empty_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "").unwrap();
        temp_file.flush().unwrap();

        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, mut rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let mut producer = ComputerFileProducer::new(state, temp_file.path().to_path_buf());

        // Should succeed but produce no objects
        let result = producer.produce().await;
        assert!(result.is_err()); // Will fail due to missing domain name

        // No objects should be received
        assert!(rx1.try_recv().is_err());
    }

    #[tokio::test]
    async fn test_produce_config_nc_no_op() {
        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let computer_file = PathBuf::from("/tmp/computers.txt");
        let mut producer = ComputerFileProducer::new(state, computer_file);

        // Configuration NC should be a no-op
        let result = producer.produce_config_nc().await;
        assert!(result.is_ok());
    }
}
