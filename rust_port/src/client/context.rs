//! Context trait and types for managing collection state
//!
//! This module defines the `Context` trait which represents the shared state
//! during a SharpHound collection run, including configuration, LDAP utilities,
//! output settings, and cancellation handling.

use crate::{EnumerationDomain, Flags, logging::BasicLogger};
use dashmap::DashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;
use tokio::task::JoinHandle;

/// Error type for file existence checks
#[derive(Debug, Error)]
#[error("File already exists: {0}")]
pub struct FileExistsError(pub String);

/// Context trait defining the shared state for collection operations
///
/// This trait is implemented by concrete context types (e.g., BaseContext)
/// and provides access to configuration, LDAP utilities, output settings,
/// and collection state.
///
/// # Thread Safety
///
/// Implementations must be Send + Sync to allow sharing across async tasks.
#[async_trait::async_trait]
pub trait Context: Send + Sync {
    // === Configuration ===

    /// Get the configuration flags
    fn flags(&self) -> &Flags;

    /// Set the configuration flags
    fn set_flags(&mut self, flags: Flags);

    // === LDAP Configuration ===

    /// LDAP filter to apply
    fn ldap_filter(&self) -> Option<&str>;

    /// Set LDAP filter
    fn set_ldap_filter(&mut self, filter: Option<String>);

    /// LDAP search base (Distinguished Name)
    fn search_base(&self) -> Option<&str>;

    /// Set LDAP search base
    fn set_search_base(&mut self, base: Option<String>);

    /// Domain name to enumerate
    fn domain_name(&self) -> Option<&str>;

    /// Set domain name
    fn set_domain_name(&mut self, domain: Option<String>);

    /// Real DNS name override
    fn real_dns_name(&self) -> Option<&str>;

    /// Set real DNS name
    fn set_real_dns_name(&mut self, name: Option<String>);

    // === Output Configuration ===

    /// Output directory path
    fn output_directory(&self) -> &PathBuf;

    /// Set output directory
    fn set_output_directory(&mut self, dir: PathBuf);

    /// Output filename prefix
    fn output_prefix(&self) -> Option<&str>;

    /// Set output prefix
    fn set_output_prefix(&mut self, prefix: Option<String>);

    /// Cache filename
    fn cache_filename(&self) -> Option<&str>;

    /// Set cache filename
    fn set_cache_filename(&mut self, filename: Option<String>);

    /// ZIP archive filename
    fn zip_filename(&self) -> Option<&str>;

    /// Set ZIP filename
    fn set_zip_filename(&mut self, filename: Option<String>);

    /// ZIP password (if password protection enabled)
    fn zip_password(&self) -> Option<&str>;

    /// Set ZIP password
    fn set_zip_password(&mut self, password: Option<String>);

    // === Enumeration Behavior ===

    /// Number of threads for enumeration
    fn threads(&self) -> usize;

    /// Set thread count
    fn set_threads(&mut self, threads: usize);

    /// Throttle delay in milliseconds
    fn throttle(&self) -> u32;

    /// Set throttle
    fn set_throttle(&mut self, throttle: u32);

    /// Jitter percentage for throttle
    fn jitter(&self) -> u32;

    /// Set jitter
    fn set_jitter(&mut self, jitter: u32);

    /// Port scan timeout in milliseconds
    fn port_scan_timeout(&self) -> u32;

    /// Set port scan timeout
    fn set_port_scan_timeout(&mut self, timeout: u32);

    /// Status update interval in milliseconds
    fn status_interval(&self) -> u32;

    /// Set status interval
    fn set_status_interval(&mut self, interval: u32);

    // === Loop Collection ===

    /// Loop duration (how long to run loop collection)
    fn loop_duration(&self) -> Option<Duration>;

    /// Set loop duration
    fn set_loop_duration(&mut self, duration: Option<Duration>);

    /// Loop interval (delay between collection cycles)
    fn loop_interval(&self) -> Option<Duration>;

    /// Set loop interval
    fn set_loop_interval(&mut self, interval: Option<Duration>);

    /// Update loop end time based on current time + duration
    fn update_loop_time(&mut self);

    // === Session/Authentication ===

    /// Current username (for session enumeration filtering)
    fn current_username(&self) -> Option<&str>;

    /// Set current username
    fn set_current_username(&mut self, username: Option<String>);

    /// Local admin username (for local admin session enum)
    fn local_admin_username(&self) -> Option<&str>;

    /// Set local admin username
    fn set_local_admin_username(&mut self, username: Option<String>);

    /// Local admin password (for local admin session enum)
    fn local_admin_password(&self) -> Option<&str>;

    /// Set local admin password
    fn set_local_admin_password(&mut self, password: Option<String>);

    // === Computer File Input ===

    /// Path to computer file (for file-based enumeration)
    fn computer_file(&self) -> Option<&str>;

    /// Set computer file path
    fn set_computer_file(&mut self, file: Option<String>);

    // === Domain Enumeration ===

    /// Domains to enumerate
    fn domains(&self) -> &[EnumerationDomain];

    /// Set domains to enumerate
    fn set_domains(&mut self, domains: Vec<EnumerationDomain>);

    /// Set of collected domain SIDs (for tracking)
    fn collected_domain_sids(&self) -> &HashSet<String>;

    /// Add a domain SID to the collected set
    fn add_collected_domain_sid(&mut self, sid: String);

    // === AdminSDHolder Hashes ===

    /// AdminSDHolder security descriptor hashes by domain
    ///
    /// Maps domain name -> SD hash for detecting AdminSDHolder-protected objects
    fn admin_sd_holder_hash(&self) -> &DashMap<String, String>;

    // === Task Management ===

    /// Collection task handle (if running)
    fn collection_task(&self) -> Option<&JoinHandle<()>>;

    /// Set collection task handle
    fn set_collection_task(&mut self, handle: Option<JoinHandle<()>>);

    /// Check if cancellation has been requested
    fn is_cancelled(&self) -> bool;

    /// Request cancellation of ongoing operations
    fn cancel(&self);

    // === Logger ===

    /// Get logger instance
    fn logger(&self) -> &BasicLogger;

    // === Utility Methods ===

    /// Apply throttle and jitter delay
    ///
    /// Calculates a random delay based on throttle ± jitter percentage
    /// and waits for that duration.
    async fn do_delay(&self);

    /// Get cache file path (resolved from cache filename or default)
    fn get_cache_path(&self) -> PathBuf;

    /// Resolve a filename with optional prefix and timestamp
    ///
    /// # Arguments
    /// * `filename` - Base filename
    /// * `extension` - File extension (without dot)
    /// * `add_timestamp` - Whether to add timestamp to filename
    ///
    /// # Returns
    /// Full path in output directory
    fn resolve_filename(&self, filename: &str, extension: &str, add_timestamp: bool) -> PathBuf;

    /// Setup collection methods for loop iteration
    ///
    /// Returns the collection methods to use for the next loop iteration,
    /// potentially filtering out methods that should only run once.
    fn setup_methods_for_loop(&self) -> Vec<String>; // Placeholder until we have CollectionMethod enum
}

/// Utility functions for context operations
pub struct ContextUtils;

impl ContextUtils {
    /// Merge two HashMaps, with values from delta overwriting those in base
    pub fn merge_maps<K, V>(
        base: std::collections::HashMap<K, V>,
        delta: std::collections::HashMap<K, V>,
    ) -> std::collections::HashMap<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        let mut result = base;
        result.extend(delta);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_file_exists_error() {
        let err = FileExistsError("test.zip".to_string());
        assert_eq!(format!("{}", err), "File already exists: test.zip");
    }

    #[test]
    fn test_context_utils_merge_maps() {
        let mut base = HashMap::new();
        base.insert("key1", "value1");
        base.insert("key2", "value2");

        let mut delta = HashMap::new();
        delta.insert("key2", "new_value2");
        delta.insert("key3", "value3");

        let result = ContextUtils::merge_maps(base, delta);

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("key1"), Some(&"value1"));
        assert_eq!(result.get("key2"), Some(&"new_value2")); // Overwritten
        assert_eq!(result.get("key3"), Some(&"value3"));
    }
}
