//! Extension methods and utility functions
//!
//! This module provides extension traits and utility functions used throughout
//! SharpHound, including dictionary merging, DNS name resolution, flag operations,
//! and async stream utilities.

use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::mpsc;

/// Extension trait for HashMap operations
pub trait HashMapExt<K, V> {
    /// Merge another HashMap into this one, adding entries that don't already exist
    ///
    /// Only adds keys from `other` that are not present in `self`.
    /// Existing keys are not overwritten.
    fn merge(&mut self, other: HashMap<K, V>);
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn merge(&mut self, other: HashMap<K, V>) {
        for (key, value) in other {
            self.entry(key).or_insert(value);
        }
    }
}

/// DNS name resolution utilities
pub struct DnsNameResolver;

impl DnsNameResolver {
    /// Get DNS name from LDAP entry properties
    ///
    /// This is a placeholder for the full implementation which requires
    /// the IDirectoryObject type from SharpHoundCommonLib.
    ///
    /// Resolution logic:
    /// 1. Use dnshostname if present
    /// 2. Fall back to samaccountname (trimmed of $) + override domain
    /// 3. Fall back to cn + override domain
    /// 4. Fall back to UNKNOWN + override domain
    ///
    /// # Arguments
    /// * `properties` - LDAP entry properties
    /// * `override_dns_name` - Fallback DNS suffix
    ///
    /// # Returns
    /// Resolved DNS name
    pub fn get_dns_name(
        properties: &HashMap<String, String>,
        override_dns_name: &str,
    ) -> String {
        // Try dnshostname first
        if let Some(dns) = properties.get("dnshostname") {
            return dns.clone();
        }

        // Get samaccountname (trimmed of $) and cn
        let short_name = properties
            .get("samaccountname")
            .map(|s| s.trim_end_matches('$'));
        let cn = properties.get("cn").map(|s| s.as_str());

        // Resolution logic
        match (short_name, cn) {
            (None, None) => format!("UNKNOWN.{}", override_dns_name),
            (Some(name), _) => format!("{}.{}", name, override_dns_name),
            (None, Some(cn)) => format!("{}.{}", cn, override_dns_name),
        }
    }
}

/// Flag extraction utilities
///
/// Provides methods for extracting individual flags from bitflag enums.
/// This is a simplified version - full implementation will come with
/// the CollectionMethod bitflags implementation.
pub struct FlagExtractor;

impl FlagExtractor {
    /// Get individual flags from a bitflag value
    ///
    /// # Example
    /// ```ignore
    /// let flags = CollectionMethod::Default; // Composite flag
    /// let individual = FlagExtractor::get_individual_flags(flags);
    /// // Returns: [Group, Session, Trusts, ACL, ...]
    /// ```
    pub fn get_individual_flags<T>(value: T) -> Vec<T>
    where
        T: Copy + Into<u64> + From<u64>,
    {
        let bits: u64 = value.into();
        let mut results = Vec::new();
        let mut flag: u64 = 1;

        while flag <= bits {
            if (bits & flag) != 0 {
                results.push(T::from(flag));
            }
            flag <<= 1;
        }

        results
    }
}

/// Async stream utilities
pub struct AsyncStreamExt;

impl AsyncStreamExt {
    /// Read all items from an async channel receiver
    ///
    /// Continues reading until the channel is closed.
    ///
    /// # Arguments
    /// * `receiver` - tokio mpsc receiver
    ///
    /// # Returns
    /// Vector of all received items
    pub async fn read_all<T>(mut receiver: mpsc::Receiver<T>) -> Vec<T> {
        let mut results = Vec::new();

        while let Some(item) = receiver.recv().await {
            results.push(item);
        }

        results
    }

    /// Read all items from a channel with a limit
    ///
    /// Stops after receiving `limit` items or when channel closes.
    ///
    /// # Arguments
    /// * `receiver` - tokio mpsc receiver
    /// * `limit` - Maximum number of items to read
    ///
    /// # Returns
    /// Vector of received items (up to limit)
    pub async fn read_n<T>(receiver: &mut mpsc::Receiver<T>, limit: usize) -> Vec<T> {
        let mut results = Vec::with_capacity(limit);

        for _ in 0..limit {
            if let Some(item) = receiver.recv().await {
                results.push(item);
            } else {
                break;
            }
        }

        results
    }
}

/// Collection method utilities
///
/// Placeholder for collection method flag operations.
/// Full implementation will come with CollectionMethod bitflags.
pub struct CollectionMethodExt;

impl CollectionMethodExt {
    /// Get collection methods suitable for loop iteration
    ///
    /// Filters to computer-specific collection methods:
    /// - LocalGroups
    /// - LoggedOn
    /// - Session
    ///
    /// This is a placeholder until CollectionMethod bitflags are implemented.
    pub fn get_loop_collection_methods(methods: &[String]) -> Vec<String> {
        const COMPUTER_METHODS: &[&str] = &["LocalGroups", "LoggedOn", "Session"];

        methods
            .iter()
            .filter(|m| COMPUTER_METHODS.contains(&m.as_str()))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_merge() {
        let mut map1 = HashMap::new();
        map1.insert("key1", "value1");
        map1.insert("key2", "value2");

        let mut map2 = HashMap::new();
        map2.insert("key2", "new_value2"); // Should not overwrite
        map2.insert("key3", "value3");

        map1.merge(map2);

        assert_eq!(map1.len(), 3);
        assert_eq!(map1.get("key1"), Some(&"value1"));
        assert_eq!(map1.get("key2"), Some(&"value2")); // Original value preserved
        assert_eq!(map1.get("key3"), Some(&"value3"));
    }

    #[test]
    fn test_get_dns_name_with_dnshostname() {
        let mut properties = HashMap::new();
        properties.insert("dnshostname".to_string(), "dc01.example.com".to_string());
        properties.insert("samaccountname".to_string(), "DC01$".to_string());

        let dns_name = DnsNameResolver::get_dns_name(&properties, "fallback.com");
        assert_eq!(dns_name, "dc01.example.com");
    }

    #[test]
    fn test_get_dns_name_with_samaccountname() {
        let mut properties = HashMap::new();
        properties.insert("samaccountname".to_string(), "COMPUTER1$".to_string());

        let dns_name = DnsNameResolver::get_dns_name(&properties, "example.com");
        assert_eq!(dns_name, "COMPUTER1.example.com");
    }

    #[test]
    fn test_get_dns_name_with_cn() {
        let mut properties = HashMap::new();
        properties.insert("cn".to_string(), "Server1".to_string());

        let dns_name = DnsNameResolver::get_dns_name(&properties, "example.com");
        assert_eq!(dns_name, "Server1.example.com");
    }

    #[test]
    fn test_get_dns_name_fallback_unknown() {
        let properties = HashMap::new();

        let dns_name = DnsNameResolver::get_dns_name(&properties, "example.com");
        assert_eq!(dns_name, "UNKNOWN.example.com");
    }

    #[test]
    fn test_get_individual_flags() {
        // Test with simple bitflags
        let flags: u64 = 0b1101; // bits 0, 2, 3 set
        let individual = FlagExtractor::get_individual_flags(flags);

        assert_eq!(individual.len(), 3);
        assert!(individual.contains(&1)); // bit 0
        assert!(individual.contains(&4)); // bit 2
        assert!(individual.contains(&8)); // bit 3
    }

    #[tokio::test]
    async fn test_read_all_from_channel() {
        let (tx, rx) = mpsc::channel(10);

        // Send some items
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
        drop(tx); // Close channel

        let results = AsyncStreamExt::read_all(rx).await;
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_read_n_from_channel() {
        let (tx, mut rx) = mpsc::channel(10);

        // Send more items than limit
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
        tx.send(4).await.unwrap();
        tx.send(5).await.unwrap();

        let results = AsyncStreamExt::read_n(&mut rx, 3).await;
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_loop_collection_methods() {
        let methods = vec![
            "Group".to_string(),
            "LocalGroups".to_string(),
            "Session".to_string(),
            "ACL".to_string(),
            "LoggedOn".to_string(),
        ];

        let loop_methods = CollectionMethodExt::get_loop_collection_methods(&methods);

        assert_eq!(loop_methods.len(), 3);
        assert!(loop_methods.contains(&"LocalGroups".to_string()));
        assert!(loop_methods.contains(&"Session".to_string()));
        assert!(loop_methods.contains(&"LoggedOn".to_string()));
        assert!(!loop_methods.contains(&"Group".to_string()));
        assert!(!loop_methods.contains(&"ACL".to_string()));
    }
}
