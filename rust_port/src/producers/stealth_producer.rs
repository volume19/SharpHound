//! Stealth producer
//!
//! This producer implements stealth collection mode, which minimizes network traffic
//! by targeting only computers that are likely to have useful data (file servers, etc.).

use super::base_producer::{BaseProducer, BaseProducerState};
use super::common_lib_placeholder::{DirectoryObject, CommonFilters, CommonProperties};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag for stealth targets built state
///
/// In the C# version, this is a static field. In Rust, we use an atomic bool
/// wrapped in a lazy_static or similar pattern. For simplicity, we'll track
/// this per-producer instance for now.
static STEALTH_TARGETS_BUILT: AtomicBool = AtomicBool::new(false);

/// Stealth producer
///
/// Implements stealth collection mode which:
/// 1. Finds computers likely to have sessions (from user home directories, scripts, profiles)
/// 2. Optionally includes domain controllers
/// 3. Queries only these targeted computers instead of all computers
///
/// This reduces network traffic and is less likely to be detected.
pub struct StealthProducer {
    /// Common producer state
    state: BaseProducerState,

    /// Cached LDAP query properties
    props: Vec<String>,

    /// Cached LDAP query properties for Configuration NC
    props_config_nc: Vec<String>,

    /// Cached LDAP filter
    query_filter: String,

    /// Cached LDAP filter for Configuration NC
    query_filter_config_nc: String,
}

impl StealthProducer {
    /// Create a new stealth producer
    ///
    /// # Arguments
    ///
    /// * `state` - Producer state (context + channels)
    pub fn new(state: BaseProducerState) -> Self {
        let ldap_data = BaseProducerState::new(
            state.context.clone(),
            state.channel.clone(),
            state.output_channel.clone(),
            state.comp_status_channel.clone(),
        );

        // Get default NC data
        let default_data = {
            let mut filter = super::common_lib_placeholder::LdapFilter::new();
            filter.add_filter(
                "(|(objectClass=user)(objectClass=computer)(objectClass=group))",
                true,
            );
            super::common_lib_placeholder::GeneratedLdapParameters::new(
                filter,
                vec!["distinguishedname".to_string(), "objectsid".to_string()],
            )
        };

        let query_filter = default_data.filter.get_filter();
        let props = default_data.attributes;

        // Get config NC data
        let config_data = {
            let mut filter = super::common_lib_placeholder::LdapFilter::new();
            filter.add_filter("(objectClass=pKICertificateTemplate)", true);
            super::common_lib_placeholder::GeneratedLdapParameters::new(
                filter,
                vec!["distinguishedname".to_string()],
            )
        };

        let query_filter_config_nc = config_data.filter.get_filter();
        let props_config_nc = config_data.attributes;

        StealthProducer {
            state,
            props,
            props_config_nc,
            query_filter,
            query_filter_config_nc,
        }
    }

    /// Build stealth targets
    ///
    /// Finds computers to target based on:
    /// - User home directories, script paths, profile paths
    /// - Domain controllers (unless excluded)
    async fn build_stealth_targets(&mut self) -> Result<()> {
        tracing::info!("Finding Stealth Targets from LDAP Properties");

        let mut targets = self.find_path_target_sids().await?;

        if !self.state.context.flags().exclude_domain_controllers {
            let dcs = self.find_domain_controllers().await?;
            targets.extend(dcs);
        }

        // TODO: Store in StealthContext when implemented
        tracing::info!("Found {} stealth targets", targets.len());

        STEALTH_TARGETS_BUILT.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Find domain controllers
    ///
    /// Queries LDAP for computers with the domain controller flag set.
    async fn find_domain_controllers(&self) -> Result<HashMap<String, DirectoryObject>> {
        let mut result = HashMap::new();

        tracing::info!("Finding domain controllers for stealth targets");

        // TODO: Implement PagedQuery when LDAP is available
        // Query filter: CommonFilters::DomainControllers
        // This would query for computers with userAccountControl:1.2.840.113556.1.4.803:=8192

        // Placeholder
        tracing::debug!("Would query: {}", CommonFilters::domain_controllers());

        Ok(result)
    }

    /// Find path target SIDs
    ///
    /// Queries users for home directories, script paths, and profile paths,
    /// extracts computer names from UNC paths, and resolves them to SIDs.
    async fn find_path_target_sids(&self) -> Result<HashMap<String, DirectoryObject>> {
        let mut result = HashMap::new();

        tracing::info!("Finding computers from user path properties");

        // TODO: Implement LDAP query for users with path properties
        // Query: (|(homedirectory=*)(scriptpath=*)(profilepath=*))
        // Properties: homedirectory, scriptpath, profilepath, objectsid, distinguishedname

        // Extract computer names from UNC paths (\\computer\share\path)
        // Resolve each computer name to SID
        // Query LDAP for the computer object

        // Placeholder
        tracing::debug!("Would query users with path properties");
        tracing::debug!("Properties: {:?}", CommonProperties::stealth_properties());

        Ok(result)
    }
}

#[async_trait]
impl BaseProducer for StealthProducer {
    async fn produce(&mut self) -> Result<()> {
        // Build stealth targets if not already built
        if !STEALTH_TARGETS_BUILT.load(Ordering::SeqCst) {
            self.build_stealth_targets().await?;
        }

        // TODO: Implement paged LDAP query with stealth targets
        // Query using the stealth target SIDs as a filter
        // Write objects to channel

        tracing::info!("Stealth producer would query stealth targets here");
        tracing::info!("Filter: {}", self.query_filter);
        tracing::info!("Attributes: {:?}", self.props);

        Ok(())
    }

    async fn produce_config_nc(&mut self) -> Result<()> {
        // Check if we have a valid filter
        if self.query_filter_config_nc.is_empty() {
            return Ok(());
        }

        // Build stealth targets if not already built
        if !STEALTH_TARGETS_BUILT.load(Ordering::SeqCst) {
            self.build_stealth_targets().await?;
        }

        // TODO: Implement paged LDAP query for Configuration NC
        // Query configuration NC objects
        // Write objects to channel

        tracing::info!("Stealth producer would query Configuration NC here");
        tracing::info!("Filter: {}", self.query_filter_config_nc);
        tracing::info!("Attributes: {:?}", self.props_config_nc);

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
    use tokio::sync::mpsc;

    #[test]
    fn test_stealth_producer_create() {
        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let _producer = StealthProducer::new(state);
    }

    #[tokio::test]
    async fn test_stealth_producer_produce() {
        // Reset global state
        STEALTH_TARGETS_BUILT.store(false, Ordering::SeqCst);

        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let mut producer = StealthProducer::new(state);

        let result = producer.produce().await;
        assert!(result.is_ok());

        // Stealth targets should now be built
        assert!(STEALTH_TARGETS_BUILT.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_stealth_producer_config_nc_empty_filter() {
        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let mut producer = StealthProducer::new(state);

        // Clear the config NC filter
        producer.query_filter_config_nc = String::new();

        let result = producer.produce_config_nc().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_domain_controllers() {
        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let producer = StealthProducer::new(state);

        let result = producer.find_domain_controllers().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Placeholder returns empty
    }

    #[tokio::test]
    async fn test_find_path_target_sids() {
        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn crate::client::Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        let producer = StealthProducer::new(state);

        let result = producer.find_path_target_sids().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Placeholder returns empty
    }
}
