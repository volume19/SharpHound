//! Base producer trait for LDAP data producers
//!
//! This module defines the producer interface for collecting Active Directory data.
//! Producers retrieve directory objects from various sources (LDAP, files, etc.)
//! and write them to channels for processing.

use super::common_lib_placeholder::{
    DirectoryObject, GeneratedLdapParameters, LdapFilter, OutputBase,
};
use crate::writers::CsvComputerStatus;
use crate::client::Context;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Base producer trait for collecting Active Directory data
///
/// Producers are responsible for:
/// - Querying LDAP or other sources for directory objects
/// - Converting raw data to DirectoryObject instances
/// - Writing objects to channels for downstream processing
/// - Handling both default and configuration naming contexts
///
/// # Naming Contexts
///
/// Active Directory has multiple naming contexts:
/// - **Default NC**: Domain-specific objects (users, computers, groups)
/// - **Configuration NC**: Forest-wide configuration (sites, services, certificate templates)
/// - **Schema NC**: AD schema definitions
///
/// Producers implement separate methods for default and configuration NCs.
#[async_trait]
pub trait BaseProducer: Send + Sync {
    /// Produce directory objects from the default naming context
    ///
    /// This method queries the default naming context for domain-specific objects
    /// like users, computers, groups, and OUs. It writes objects to the channel
    /// for downstream processing.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if production fails
    async fn produce(&mut self) -> Result<()>;

    /// Produce directory objects from the configuration naming context
    ///
    /// This method queries the configuration naming context for forest-wide objects
    /// like PKI objects (certificate templates, CAs), sites, and services.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if production fails
    async fn produce_config_nc(&mut self) -> Result<()>;

    /// Get the context
    fn context(&self) -> &Arc<dyn Context>;

    /// Create default naming context query parameters
    ///
    /// Generates LDAP filter and attributes for the default NC based on
    /// collection methods specified in the context.
    ///
    /// # Returns
    ///
    /// Generated LDAP parameters (filter + attributes)
    fn create_default_nc_data(&self) -> GeneratedLdapParameters {
        // TODO: Implement LdapProducerQueryGenerator when SharpHoundCommonLib is ported
        // For now, return a placeholder
        let mut filter = LdapFilter::new();

        // Add basic filters based on collection methods
        // This is a simplified version - full implementation will use
        // LdapProducerQueryGenerator.GenerateDefaultPartitionParameters()
        filter.add_filter("(|(objectClass=user)(objectClass=computer)(objectClass=group)(objectClass=organizationalUnit))", true);

        // Apply custom LDAP filter if specified
        if let Some(custom_filter) = self.context().ldap_filter() {
            filter.add_filter(custom_filter, true);
        }

        GeneratedLdapParameters::new(
            filter,
            vec![
                "distinguishedname".to_string(),
                "objectsid".to_string(),
                "objectclass".to_string(),
                "samaccountname".to_string(),
            ],
        )
    }

    /// Create configuration naming context query parameters
    ///
    /// Generates LDAP filter and attributes for the configuration NC based on
    /// collection methods specified in the context.
    ///
    /// # Returns
    ///
    /// Generated LDAP parameters (filter + attributes)
    fn create_config_nc_data(&self) -> GeneratedLdapParameters {
        // TODO: Implement LdapProducerQueryGenerator when SharpHoundCommonLib is ported
        // For now, return a placeholder
        let mut filter = LdapFilter::new();

        // Add basic filters for configuration NC objects
        // This is a simplified version - full implementation will use
        // LdapProducerQueryGenerator.GenerateConfigurationPartitionParameters()
        filter.add_filter(
            "(|(objectClass=pKICertificateTemplate)(objectClass=pKIEnrollmentService))",
            true,
        );

        // Apply custom LDAP filter if specified
        if let Some(custom_filter) = self.context().ldap_filter() {
            filter.add_filter(custom_filter, true);
        }

        GeneratedLdapParameters::new(
            filter,
            vec![
                "distinguishedname".to_string(),
                "objectguid".to_string(),
                "objectclass".to_string(),
                "name".to_string(),
            ],
        )
    }
}

/// Common state for producers
///
/// This struct holds the common state shared by all producer implementations.
/// It manages channels for communication with downstream processors.
pub struct BaseProducerState {
    /// Context for the collection run
    pub context: Arc<dyn Context>,

    /// Channel for writing directory objects
    pub channel: mpsc::Sender<DirectoryObject>,

    /// Channel for writing processed output
    pub output_channel: mpsc::Sender<OutputBase>,

    /// Channel for writing computer status updates
    pub comp_status_channel: mpsc::Sender<CsvComputerStatus>,
}

impl BaseProducerState {
    /// Create a new producer state
    ///
    /// # Arguments
    ///
    /// * `context` - The collection context
    /// * `channel` - Channel for directory objects
    /// * `output_channel` - Channel for processed output
    /// * `comp_status_channel` - Channel for computer status
    pub fn new(
        context: Arc<dyn Context>,
        channel: mpsc::Sender<DirectoryObject>,
        output_channel: mpsc::Sender<OutputBase>,
        comp_status_channel: mpsc::Sender<CsvComputerStatus>,
    ) -> Self {
        BaseProducerState {
            context,
            channel,
            output_channel,
            comp_status_channel,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_ldap_parameters() {
        let filter = LdapFilter::new();
        let attributes = vec!["cn".to_string(), "dn".to_string()];
        let params = GeneratedLdapParameters::new(filter, attributes.clone());

        assert_eq!(params.attributes.len(), 2);
        assert_eq!(params.attributes[0], "cn");
    }

    #[test]
    fn test_base_producer_state_new() {
        use crate::BaseContext;
        use crate::Flags;

        let context = Arc::new(BaseContext::new_for_test(Flags::default())) as Arc<dyn Context>;
        let (tx1, _rx1) = mpsc::channel(100);
        let (tx2, _rx2) = mpsc::channel(100);
        let (tx3, _rx3) = mpsc::channel(100);

        let state = BaseProducerState::new(context, tx1, tx2, tx3);
        assert!(state.context.flags().stealth == false);
    }
}
