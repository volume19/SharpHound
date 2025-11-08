//! Domain enumeration types
//!
//! This module defines types used for representing Active Directory domains
//! during enumeration, including domain names, SIDs, and trust relationships.

use serde::{Deserialize, Serialize};

/// Represents a domain to be enumerated
///
/// Contains identifying information about an Active Directory domain,
/// including its DNS name, security identifier (SID), and trust type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumerationDomain {
    /// DNS name of the domain (e.g., "example.com")
    pub name: String,

    /// Security Identifier (SID) of the domain (e.g., "S-1-5-21-...")
    pub domain_sid: String,

    /// Type of trust relationship with this domain
    /// (e.g., "ParentChild", "External", "Forest", "Unknown")
    pub trust_type: String,
}

impl EnumerationDomain {
    /// Create a new enumeration domain
    pub fn new(name: impl Into<String>, domain_sid: impl Into<String>, trust_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            domain_sid: domain_sid.into(),
            trust_type: trust_type.into(),
        }
    }

    /// Check if this domain SID is valid (basic format check)
    pub fn has_valid_sid(&self) -> bool {
        self.domain_sid.starts_with("S-1-5-21-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumeration_domain_new() {
        let domain = EnumerationDomain::new(
            "example.com",
            "S-1-5-21-1234567890-1234567890-1234567890",
            "ParentChild",
        );

        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.domain_sid, "S-1-5-21-1234567890-1234567890-1234567890");
        assert_eq!(domain.trust_type, "ParentChild");
    }

    #[test]
    fn test_has_valid_sid() {
        let valid_domain = EnumerationDomain::new(
            "example.com",
            "S-1-5-21-1234567890-1234567890-1234567890",
            "Forest",
        );
        assert!(valid_domain.has_valid_sid());

        let invalid_domain = EnumerationDomain::new(
            "example.com",
            "INVALID-SID",
            "External",
        );
        assert!(!invalid_domain.has_valid_sid());
    }

    #[test]
    fn test_serialize_deserialize() {
        let domain = EnumerationDomain::new(
            "test.local",
            "S-1-5-21-111-222-333",
            "Unknown",
        );

        let json = serde_json::to_string(&domain).unwrap();
        let deserialized: EnumerationDomain = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, domain);
    }

    #[test]
    fn test_clone() {
        let domain = EnumerationDomain::new("domain.local", "S-1-5-21-1-2-3", "Forest");
        let cloned = domain.clone();

        assert_eq!(cloned, domain);
        assert_eq!(cloned.name, domain.name);
    }
}
