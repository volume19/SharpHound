//! Configuration flags for SharpHound collection behavior
//!
//! This module defines the `Flags` struct which contains boolean flags
//! controlling various aspects of the collection process.

use serde::{Deserialize, Serialize};

/// Configuration flags controlling collection behavior
///
/// These flags are set based on command-line options and control
/// how SharpHound performs enumeration, output generation, and
/// LDAP connections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flags {
    /// Enable stealth collection mode (reduces network noise)
    pub stealth: bool,

    /// Whether the initial collection phase has completed
    pub initial_completed: bool,

    /// Whether a cancellation has been requested
    pub needs_cancellation: bool,

    /// Enable loop collection mode
    pub loop_mode: bool,

    /// Whether an error has occurred
    pub is_faulted: bool,

    /// Disable output generation
    pub no_output: bool,

    /// Use random filenames for output
    pub randomize_filenames: bool,

    /// Keep cache in memory only (don't write to disk)
    pub mem_cache: bool,

    /// Don't create ZIP archive of output
    pub no_zip: bool,

    /// Invalidate/rebuild cache
    pub invalidate_cache: bool,

    /// Use LDAPS (secure LDAP) connection
    pub secure_ldap: bool,

    /// Disable Kerberos signing/sealing
    pub disable_kerberos_signing: bool,

    /// Skip port 445 (SMB) scan check
    pub skip_port_scan: bool,

    /// Skip password age check for computers
    pub skip_password_age_check: bool,

    /// Exclude domain controllers from enumeration
    pub exclude_domain_controllers: bool,

    /// Skip registry-based logged-on user enumeration
    pub no_registry_logged_on: bool,

    /// Output computer status CSV
    pub dump_computer_status: bool,

    /// Collect all LDAP properties (not just standard set)
    pub collect_all_properties: bool,

    /// DC-only collection mode (LDAP-focused)
    pub dc_only: bool,

    /// Pretty-print JSON output
    pub pretty_print: bool,

    /// Search entire forest for domains
    pub search_forest: bool,

    /// Recurse through domain trusts
    pub recurse_domains: bool,

    /// Use local admin credentials for session enumeration
    pub do_local_admin_session_enum: bool,

    /// Partition LDAP queries into smaller chunks (Note: typo preserved from C# for compatibility)
    #[serde(rename = "ParititonLdapQueries")]
    pub parititon_ldap_queries: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            stealth: false,
            initial_completed: false,
            needs_cancellation: false,
            loop_mode: false,
            is_faulted: false,
            no_output: false,
            randomize_filenames: false,
            mem_cache: false,
            no_zip: false,
            invalidate_cache: false,
            secure_ldap: false,
            disable_kerberos_signing: false,
            skip_port_scan: false,
            skip_password_age_check: false,
            exclude_domain_controllers: false,
            no_registry_logged_on: false,
            dump_computer_status: false,
            collect_all_properties: false,
            dc_only: false,
            pretty_print: false,
            search_forest: false,
            recurse_domains: false,
            do_local_admin_session_enum: false,
            parititon_ldap_queries: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_default() {
        let flags = Flags::default();
        assert!(!flags.stealth);
        assert!(!flags.loop_mode);
        assert!(!flags.is_faulted);
        assert!(!flags.dc_only);
    }

    #[test]
    fn test_flags_builder_pattern() {
        let mut flags = Flags::default();
        flags.stealth = true;
        flags.dc_only = true;
        flags.pretty_print = true;

        assert!(flags.stealth);
        assert!(flags.dc_only);
        assert!(flags.pretty_print);
        assert!(!flags.loop_mode);
    }

    #[test]
    fn test_flags_serialize_deserialize() {
        let mut flags = Flags::default();
        flags.stealth = true;
        flags.collect_all_properties = true;

        let json = serde_json::to_string(&flags).unwrap();
        let deserialized: Flags = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, flags);
        assert!(deserialized.stealth);
        assert!(deserialized.collect_all_properties);
    }

    #[test]
    fn test_flags_clone() {
        let mut flags = Flags::default();
        flags.loop_mode = true;
        flags.randomize_filenames = true;

        let cloned = flags.clone();
        assert_eq!(cloned, flags);
        assert!(cloned.loop_mode);
        assert!(cloned.randomize_filenames);
    }
}
