//! Chain of Responsibility pattern for collection workflow
//!
//! This module defines the `Links` trait which implements the Chain of Responsibility
//! pattern for executing SharpHound's collection workflow steps in sequence.

use crate::client::Context;
use anyhow::Result;

/// LDAP connection configuration
///
/// This is a simplified version of SharpHoundCommonLib's LdapConfig.
/// Full implementation will come with LDAP integration.
#[derive(Debug, Clone)]
pub struct LdapConfig {
    /// LDAP server (domain controller) to connect to
    pub server: Option<String>,

    /// LDAP port (default: 389, or 636 for SSL)
    pub port: u16,

    /// LDAPS port (default: 636)
    pub ssl_port: u16,

    /// Force SSL/TLS connection
    pub force_ssl: bool,

    /// Disable certificate verification (insecure)
    pub disable_cert_verification: bool,

    /// Disable Kerberos signing
    pub disable_signing: bool,

    /// Username for authentication (if not using current credentials)
    pub username: Option<String>,

    /// Password for authentication
    pub password: Option<String>,

    /// Authentication type (defaults to Negotiate)
    pub auth_type: AuthType,
}

/// LDAP authentication type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthType {
    /// Negotiate (Kerberos or NTLM)
    Negotiate,
    /// Simple bind (plaintext password)
    Simple,
    /// Anonymous
    Anonymous,
}

impl Default for LdapConfig {
    fn default() -> Self {
        Self {
            server: None,
            port: 389,
            ssl_port: 636,
            force_ssl: false,
            disable_cert_verification: false,
            disable_signing: false,
            username: None,
            password: None,
            auth_type: AuthType::Negotiate,
        }
    }
}

/// Links trait defining the Chain of Responsibility workflow
///
/// This trait implements the step-by-step execution pattern for SharpHound collection:
/// 1. Initialize and validate configuration
/// 2. Test LDAP connection
/// 3. Set up session username
/// 4. Initialize common library components
/// 5. Get domains for enumeration
/// 6. Start base collection task
/// 7. Await base collection completion
/// 8. Start loop timer (if loop mode enabled)
/// 9. Start loop collection
/// 10. Await loop completion
/// 11. Dispose timer
/// 12. Save cache file
/// 13. Finish and cleanup
///
/// Each method takes a context and returns a modified context,
/// allowing the workflow to pass state through the chain.
#[async_trait::async_trait]
pub trait Links<T: Context> {
    /// Initialize the collection context
    ///
    /// Sets up initial configuration, validates options, and prepares
    /// the context for collection operations.
    ///
    /// # Arguments
    /// * `context` - The context to initialize
    /// * `config` - LDAP connection configuration
    ///
    /// # Returns
    /// Modified context with initialization complete
    fn initialize(&self, context: T, config: LdapConfig) -> Result<T>;

    /// Test LDAP connection
    ///
    /// Performs an initial LDAP query to verify connectivity.
    /// Typically searches for the well-known administrator SID.
    ///
    /// # Arguments
    /// * `context` - The context with LDAP configuration
    ///
    /// # Returns
    /// Modified context with connection verified
    async fn test_connection(&self, context: T) -> Result<T>;

    /// Set session username for filtering
    ///
    /// Sets the current username to filter NetSessionEnum results.
    /// Can be overridden for specific enumeration scenarios.
    ///
    /// # Arguments
    /// * `override_username` - Optional username override
    /// * `context` - The context to modify
    ///
    /// # Returns
    /// Modified context with username set
    fn set_session_username(&self, override_username: Option<String>, context: T) -> Result<T>;

    /// Initialize common library components
    ///
    /// Sets up SharpHoundCommonLib utilities, cache, and processors.
    ///
    /// # Arguments
    /// * `context` - The context to initialize
    ///
    /// # Returns
    /// Modified context with common lib initialized
    fn init_common_lib(&self, context: T) -> Result<T>;

    /// Get domains for enumeration
    ///
    /// Queries Active Directory to build the list of domains to enumerate.
    /// Handles forest enumeration, trust relationships, and domain recursion.
    ///
    /// # Arguments
    /// * `context` - The context to populate with domains
    ///
    /// # Returns
    /// Modified context with domains list populated
    async fn get_domains_for_enumeration(&self, context: T) -> Result<T>;

    /// Start base collection task
    ///
    /// Spawns the main collection task that enumerates Active Directory objects.
    ///
    /// # Arguments
    /// * `context` - The context to use for collection
    ///
    /// # Returns
    /// Modified context with collection task handle
    fn start_base_collection_task(&self, context: T) -> Result<T>;

    /// Await base collection completion
    ///
    /// Waits for the main collection task to complete.
    ///
    /// # Arguments
    /// * `context` - The context with running collection task
    ///
    /// # Returns
    /// Modified context after collection completes
    async fn await_base_run_completion(&self, context: T) -> Result<T>;

    /// Start loop timer
    ///
    /// Initializes the timer for loop collection mode.
    ///
    /// # Arguments
    /// * `context` - The context to set up loop timing
    ///
    /// # Returns
    /// Modified context with loop timer started
    fn start_loop_timer(&self, context: T) -> Result<T>;

    /// Start loop collection
    ///
    /// Begins loop collection if enabled, repeatedly running collection
    /// cycles at specified intervals.
    ///
    /// # Arguments
    /// * `context` - The context for loop collection
    ///
    /// # Returns
    /// Modified context with loop running
    fn start_loop(&self, context: T) -> Result<T>;

    /// Await loop completion
    ///
    /// Waits for all loop collection cycles to complete.
    ///
    /// # Arguments
    /// * `context` - The context with running loop
    ///
    /// # Returns
    /// Modified context after loop completes
    async fn await_loop_completion(&self, context: T) -> Result<T>;

    /// Dispose timer
    ///
    /// Cleans up the loop timer resources.
    ///
    /// # Arguments
    /// * `context` - The context with timer to dispose
    ///
    /// # Returns
    /// Modified context with timer disposed
    fn dispose_timer(&self, context: T) -> Result<T>;

    /// Save cache file
    ///
    /// Writes the collection cache to disk (unless memcache enabled).
    ///
    /// # Arguments
    /// * `context` - The context with cache to save
    ///
    /// # Returns
    /// Modified context with cache saved
    fn save_cache_file(&self, context: T) -> Result<T>;

    /// Finish collection and cleanup
    ///
    /// Performs final cleanup, closes connections, and prepares for exit.
    ///
    /// # Arguments
    /// * `context` - The context to finish
    ///
    /// # Returns
    /// Final context state
    fn finish(&self, context: T) -> Result<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldap_config_default() {
        let config = LdapConfig::default();
        assert_eq!(config.port, 389);
        assert_eq!(config.ssl_port, 636);
        assert!(!config.force_ssl);
        assert!(!config.disable_cert_verification);
        assert_eq!(config.auth_type, AuthType::Negotiate);
    }

    #[test]
    fn test_ldap_config_builder() {
        let mut config = LdapConfig::default();
        config.server = Some("dc01.example.com".to_string());
        config.force_ssl = true;
        config.port = 636;

        assert_eq!(config.server, Some("dc01.example.com".to_string()));
        assert!(config.force_ssl);
        assert_eq!(config.port, 636);
    }

    #[test]
    fn test_auth_type_equality() {
        assert_eq!(AuthType::Negotiate, AuthType::Negotiate);
        assert_ne!(AuthType::Simple, AuthType::Anonymous);
    }
}
