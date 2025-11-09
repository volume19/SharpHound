//! BaseContext - Concrete implementation of the Context trait
//!
//! This module provides the main context implementation used during
//! SharpHound collection operations.

use crate::client::{Context, LdapConfig};
use crate::{EnumerationDomain, Flags, logging::BasicLogger};
use base64::Engine;
use chrono::Local;
use dashmap::DashMap;
use rand::Rng;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

/// BaseContext - Main context implementation for SharpHound collection
///
/// This struct implements the `Context` trait and maintains all state
/// during a collection run, including configuration, LDAP utilities,
/// output settings, and collection tracking.
pub struct BaseContext {
    // Configuration
    flags: Flags,

    // LDAP Configuration
    ldap_filter: Option<String>,
    search_base: Option<String>,
    domain_name: Option<String>,
    real_dns_name: Option<String>,

    // Output Configuration
    output_directory: PathBuf,
    output_prefix: Option<String>,
    cache_filename: Option<String>,
    zip_filename: Option<String>,
    zip_password: Option<String>,

    // Enumeration Behavior
    threads: usize,
    throttle: u32,
    jitter: u32,
    port_scan_timeout: u32,
    status_interval: u32,

    // Loop Collection
    loop_duration: Option<Duration>,
    loop_interval: Option<Duration>,
    current_loop_time: Arc<RwLock<String>>,

    // Session/Authentication
    current_username: Option<String>,
    local_admin_username: Option<String>,
    local_admin_password: Option<String>,

    // Computer File Input
    computer_file: Option<String>,

    // Domain Enumeration
    domains: Vec<EnumerationDomain>,
    collected_domain_sids: HashSet<String>,

    // AdminSDHolder Hashes (concurrent access)
    admin_sd_holder_hash: Arc<DashMap<String, String>>,

    // Task Management
    collection_task: Option<JoinHandle<()>>,
    is_cancelled: Arc<RwLock<bool>>,

    // Logger
    logger: BasicLogger,

    // LDAP Config (stored for reference)
    ldap_config: LdapConfig,
}

impl BaseContext {
    /// Create a new BaseContext
    ///
    /// # Arguments
    /// * `logger` - Logger instance for output
    /// * `ldap_config` - LDAP connection configuration
    /// * `flags` - Configuration flags
    pub fn new(logger: BasicLogger, ldap_config: LdapConfig, flags: Flags) -> Self {
        let now = Local::now();
        let current_loop_time = format!("{}", now.format("%Y%m%d%H%M%S"));

        Self {
            flags,
            ldap_filter: None,
            search_base: None,
            domain_name: None,
            real_dns_name: None,
            output_directory: PathBuf::from("."),
            output_prefix: None,
            cache_filename: None,
            zip_filename: None,
            zip_password: None,
            threads: 50,
            throttle: 0,
            jitter: 0,
            port_scan_timeout: 500,
            status_interval: 30000,
            loop_duration: None,
            loop_interval: None,
            current_loop_time: Arc::new(RwLock::new(current_loop_time)),
            current_username: None,
            local_admin_username: None,
            local_admin_password: None,
            computer_file: None,
            domains: Vec::new(),
            collected_domain_sids: HashSet::new(),
            admin_sd_holder_hash: Arc::new(DashMap::new()),
            collection_task: None,
            is_cancelled: Arc::new(RwLock::new(false)),
            logger,
            ldap_config,
        }
    }

    /// Get machine-specific cache filename
    ///
    /// Generates a machine-specific identifier for the cache file.
    /// In the C# version this uses the Windows MachineGuid from registry.
    /// For cross-platform, we'll use a simpler approach.
    fn get_machine_id() -> String {
        // TODO: Implement platform-specific machine ID
        // Windows: Read from HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid
        // Linux: Read from /etc/machine-id or generate UUID
        // For now, use a simple fallback
        use uuid::Uuid;
        let id = Uuid::new_v4();
        base64::engine::general_purpose::STANDARD.encode(id.as_bytes())
    }
}

#[async_trait::async_trait]
impl Context for BaseContext {
    // === Configuration ===

    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }

    // === LDAP Configuration ===

    fn ldap_filter(&self) -> Option<&str> {
        self.ldap_filter.as_deref()
    }

    fn set_ldap_filter(&mut self, filter: Option<String>) {
        self.ldap_filter = filter;
    }

    fn search_base(&self) -> Option<&str> {
        self.search_base.as_deref()
    }

    fn set_search_base(&mut self, base: Option<String>) {
        self.search_base = base;
    }

    fn domain_name(&self) -> Option<&str> {
        self.domain_name.as_deref()
    }

    fn set_domain_name(&mut self, domain: Option<String>) {
        self.domain_name = domain;
    }

    fn real_dns_name(&self) -> Option<&str> {
        self.real_dns_name.as_deref()
    }

    fn set_real_dns_name(&mut self, name: Option<String>) {
        self.real_dns_name = name;
    }

    // === Output Configuration ===

    fn output_directory(&self) -> &PathBuf {
        &self.output_directory
    }

    fn set_output_directory(&mut self, dir: PathBuf) {
        self.output_directory = dir;
    }

    fn output_prefix(&self) -> Option<&str> {
        self.output_prefix.as_deref()
    }

    fn set_output_prefix(&mut self, prefix: Option<String>) {
        self.output_prefix = prefix;
    }

    fn cache_filename(&self) -> Option<&str> {
        self.cache_filename.as_deref()
    }

    fn set_cache_filename(&mut self, filename: Option<String>) {
        self.cache_filename = filename;
    }

    fn zip_filename(&self) -> Option<&str> {
        self.zip_filename.as_deref()
    }

    fn set_zip_filename(&mut self, filename: Option<String>) {
        self.zip_filename = filename;
    }

    fn zip_password(&self) -> Option<&str> {
        self.zip_password.as_deref()
    }

    fn set_zip_password(&mut self, password: Option<String>) {
        self.zip_password = password;
    }

    // === Enumeration Behavior ===

    fn threads(&self) -> usize {
        self.threads
    }

    fn set_threads(&mut self, threads: usize) {
        self.threads = threads;
    }

    fn throttle(&self) -> u32 {
        self.throttle
    }

    fn set_throttle(&mut self, throttle: u32) {
        self.throttle = throttle;
    }

    fn jitter(&self) -> u32 {
        self.jitter
    }

    fn set_jitter(&mut self, jitter: u32) {
        self.jitter = jitter;
    }

    fn port_scan_timeout(&self) -> u32 {
        self.port_scan_timeout
    }

    fn set_port_scan_timeout(&mut self, timeout: u32) {
        self.port_scan_timeout = timeout;
    }

    fn status_interval(&self) -> u32 {
        self.status_interval
    }

    fn set_status_interval(&mut self, interval: u32) {
        self.status_interval = interval;
    }

    // === Loop Collection ===

    fn loop_duration(&self) -> Option<Duration> {
        self.loop_duration
    }

    fn set_loop_duration(&mut self, duration: Option<Duration>) {
        self.loop_duration = duration;
    }

    fn loop_interval(&self) -> Option<Duration> {
        self.loop_interval
    }

    fn set_loop_interval(&mut self, interval: Option<Duration>) {
        self.loop_interval = interval;
    }

    fn update_loop_time(&mut self) {
        let now = Local::now();
        let time_string = format!("{}", now.format("%Y%m%d%H%M%S"));
        // Note: This is blocking but we're in a sync method
        // In real usage, prefer async update_loop_time_async
        if let Ok(mut time) = self.current_loop_time.try_write() {
            *time = time_string;
        }
    }

    // === Session/Authentication ===

    fn current_username(&self) -> Option<&str> {
        self.current_username.as_deref()
    }

    fn set_current_username(&mut self, username: Option<String>) {
        self.current_username = username;
    }

    fn local_admin_username(&self) -> Option<&str> {
        self.local_admin_username.as_deref()
    }

    fn set_local_admin_username(&mut self, username: Option<String>) {
        self.local_admin_username = username;
    }

    fn local_admin_password(&self) -> Option<&str> {
        self.local_admin_password.as_deref()
    }

    fn set_local_admin_password(&mut self, password: Option<String>) {
        self.local_admin_password = password;
    }

    // === Computer File Input ===

    fn computer_file(&self) -> Option<&str> {
        self.computer_file.as_deref()
    }

    fn set_computer_file(&mut self, file: Option<String>) {
        self.computer_file = file;
    }

    // === Domain Enumeration ===

    fn domains(&self) -> &[EnumerationDomain] {
        &self.domains
    }

    fn set_domains(&mut self, domains: Vec<EnumerationDomain>) {
        self.domains = domains;
    }

    fn collected_domain_sids(&self) -> &HashSet<String> {
        &self.collected_domain_sids
    }

    fn add_collected_domain_sid(&mut self, sid: String) {
        self.collected_domain_sids.insert(sid);
    }

    // === AdminSDHolder Hashes ===

    fn admin_sd_holder_hash(&self) -> &DashMap<String, String> {
        &self.admin_sd_holder_hash
    }

    // === Task Management ===

    fn collection_task(&self) -> Option<&JoinHandle<()>> {
        self.collection_task.as_ref()
    }

    fn set_collection_task(&mut self, handle: Option<JoinHandle<()>>) {
        self.collection_task = handle;
    }

    fn is_cancelled(&self) -> bool {
        // Try non-blocking read
        self.is_cancelled.try_read().map(|r| *r).unwrap_or(false)
    }

    fn cancel(&self) {
        // Try non-blocking write
        if let Ok(mut cancelled) = self.is_cancelled.try_write() {
            *cancelled = true;
        }
    }

    // === Logger ===

    fn logger(&self) -> &BasicLogger {
        &self.logger
    }

    // === Utility Methods ===

    /// Apply throttle and jitter delay
    ///
    /// Calculates a random delay based on throttle ± jitter percentage.
    ///
    /// Algorithm matches C# implementation:
    /// - If throttle is 0, no delay
    /// - If jitter is 0, delay exactly throttle ms
    /// - Otherwise, delay = throttle ± (jitter% of throttle)
    async fn do_delay(&self) {
        if self.throttle == 0 {
            return;
        }

        if self.jitter == 0 {
            tokio::time::sleep(Duration::from_millis(self.throttle as u64)).await;
            return;
        }

        // Calculate jitter range
        let percent = ((self.jitter as f64) * (self.throttle as f64 / 100.0)).floor() as i32;

        // Generate random jitter offset (must be in its own scope to avoid Send issue)
        let jitter_offset = {
            let mut rng = rand::thread_rng();
            rng.gen_range(-percent..=percent)
        };

        let delay = (self.throttle as i32 + jitter_offset).max(0) as u64;
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }

    /// Get cache file path
    ///
    /// Resolves the cache filename (or generates a machine-specific one)
    /// and combines it with the output directory.
    fn get_cache_path(&self) -> PathBuf {
        let cache_filename = self.cache_filename
            .as_deref()
            .unwrap_or_else(|| {
                // Generate machine-specific cache filename
                // Note: This returns a static str, so we need to handle differently
                "cache.bin" // Placeholder - see get_machine_id for TODO
            });

        self.output_directory.join(cache_filename)
    }

    /// Resolve a filename with optional prefix and timestamp
    ///
    /// # Arguments
    /// * `filename` - Base filename
    /// * `extension` - File extension (without dot)
    /// * `add_timestamp` - Whether to add timestamp to filename
    ///
    /// # Returns
    /// Full path in output directory
    ///
    /// # Behavior
    /// 1. Adds extension if not present
    /// 2. Randomizes filename if RandomizeFilenames flag set (for json/zip)
    /// 3. Prepends timestamp if add_timestamp is true
    /// 4. Prepends output prefix if set
    /// 5. Combines with output directory
    fn resolve_filename(&self, filename: &str, extension: &str, add_timestamp: bool) -> PathBuf {
        let mut final_filename = filename.to_string();

        // Add extension if not present
        if !final_filename.ends_with(extension) {
            final_filename = format!("{}.{}", final_filename, extension);
        }

        // Randomize filename if flag set (for json/zip only)
        if (extension == "json" || extension == "zip") && self.flags.randomize_filenames {
            // Generate random filename (8.3 format like Path.GetRandomFileName)
            use rand::distributions::Alphanumeric;
            use rand::Rng;
            let random_name: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();
            final_filename = format!("{}.{}", random_name, extension);
        }

        // Prepend timestamp if requested
        if add_timestamp {
            let loop_time = self.current_loop_time.try_read()
                .map(|t| t.clone())
                .unwrap_or_else(|_| {
                    let now = Local::now();
                    format!("{}", now.format("%Y%m%d%H%M%S"))
                });
            final_filename = format!("{}_{}", loop_time, final_filename);
        }

        // Prepend output prefix if set
        if let Some(prefix) = &self.output_prefix {
            final_filename = format!("{}_{}", prefix, final_filename);
        }

        // Combine with output directory
        self.output_directory.join(final_filename)
    }

    /// Setup collection methods for loop iteration
    ///
    /// Returns only the collection methods that should run in loop mode.
    /// Filters to computer-specific collection methods:
    /// - LocalGroups
    /// - LoggedOn
    /// - Session
    fn setup_methods_for_loop(&self) -> Vec<String> {
        // TODO: This needs the full CollectionMethod bitflags implementation
        // For now, return placeholder
        // In C# this does: original & (LocalGroups | LoggedOn | Session)
        vec![
            "LocalGroups".to_string(),
            "LoggedOn".to_string(),
            "Session".to_string(),
        ]
    }
}

// Implement Drop for cleanup (replaces C# Dispose pattern)
impl Drop for BaseContext {
    fn drop(&mut self) {
        // Cancel any ongoing operations
        self.cancel();

        // Task handles will be automatically cleaned up by their Drop impl
        // DashMap and other Arc types will be cleaned up when ref count reaches 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_context_new() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        assert_eq!(ctx.threads(), 50);
        assert_eq!(ctx.throttle(), 0);
        assert_eq!(ctx.jitter(), 0);
        assert_eq!(ctx.port_scan_timeout(), 500);
        assert_eq!(ctx.status_interval(), 30000);
        assert_eq!(ctx.output_directory(), &PathBuf::from("."));
    }

    #[test]
    fn test_base_context_setters() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let mut ctx = BaseContext::new(logger, ldap_config, flags);

        ctx.set_domain_name(Some("example.com".to_string()));
        ctx.set_threads(100);
        ctx.set_throttle(1000);
        ctx.set_jitter(25);

        assert_eq!(ctx.domain_name(), Some("example.com"));
        assert_eq!(ctx.threads(), 100);
        assert_eq!(ctx.throttle(), 1000);
        assert_eq!(ctx.jitter(), 25);
    }

    #[tokio::test]
    async fn test_do_delay_no_throttle() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        let start = std::time::Instant::now();
        ctx.do_delay().await;
        let elapsed = start.elapsed();

        // Should return immediately (< 10ms)
        assert!(elapsed < Duration::from_millis(10));
    }

    #[tokio::test]
    async fn test_do_delay_with_throttle_no_jitter() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let mut ctx = BaseContext::new(logger, ldap_config, flags);
        ctx.set_throttle(100);

        let start = std::time::Instant::now();
        ctx.do_delay().await;
        let elapsed = start.elapsed();

        // Should delay ~100ms (allow 90-110ms range)
        assert!(elapsed >= Duration::from_millis(90));
        assert!(elapsed <= Duration::from_millis(110));
    }

    #[test]
    fn test_get_cache_path_default() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        let path = ctx.get_cache_path();
        assert_eq!(path, PathBuf::from("./cache.bin"));
    }

    #[test]
    fn test_get_cache_path_custom() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let mut ctx = BaseContext::new(logger, ldap_config, flags);
        ctx.set_cache_filename(Some("mycache.bin".to_string()));

        let path = ctx.get_cache_path();
        assert_eq!(path, PathBuf::from("./mycache.bin"));
    }

    #[test]
    fn test_resolve_filename_basic() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        let path = ctx.resolve_filename("output", "json", false);
        assert_eq!(path, PathBuf::from("./output.json"));
    }

    #[test]
    fn test_resolve_filename_with_prefix() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let mut ctx = BaseContext::new(logger, ldap_config, flags);
        ctx.set_output_prefix(Some("test".to_string()));

        let path = ctx.resolve_filename("output", "json", false);
        assert_eq!(path, PathBuf::from("./test_output.json"));
    }

    #[test]
    fn test_resolve_filename_with_timestamp() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        let path = ctx.resolve_filename("output", "json", true);
        let filename = path.file_name().unwrap().to_str().unwrap();

        // Should start with timestamp format (14 digits)
        assert!(filename.len() > 14);
        assert!(filename.contains("_output.json"));
    }

    #[test]
    fn test_domains() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let mut ctx = BaseContext::new(logger, ldap_config, flags);

        let domains = vec![
            EnumerationDomain::new("example.com", "S-1-5-21-1-2-3", "Forest"),
            EnumerationDomain::new("test.local", "S-1-5-21-4-5-6", "External"),
        ];

        ctx.set_domains(domains.clone());
        assert_eq!(ctx.domains().len(), 2);
        assert_eq!(ctx.domains()[0].name, "example.com");
    }

    #[test]
    fn test_collected_domain_sids() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let mut ctx = BaseContext::new(logger, ldap_config, flags);

        ctx.add_collected_domain_sid("S-1-5-21-1-2-3".to_string());
        ctx.add_collected_domain_sid("S-1-5-21-4-5-6".to_string());

        assert_eq!(ctx.collected_domain_sids().len(), 2);
        assert!(ctx.collected_domain_sids().contains("S-1-5-21-1-2-3"));
    }

    #[test]
    fn test_admin_sd_holder_hash() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        ctx.admin_sd_holder_hash().insert("example.com".to_string(), "hash123".to_string());
        assert_eq!(ctx.admin_sd_holder_hash().get("example.com").map(|r| r.value().clone()), Some("hash123".to_string()));
    }

    #[test]
    fn test_cancellation() {
        let logger = BasicLogger::new(2);
        let ldap_config = LdapConfig::default();
        let flags = Flags::default();

        let ctx = BaseContext::new(logger, ldap_config, flags);

        assert!(!ctx.is_cancelled());
        ctx.cancel();
        assert!(ctx.is_cancelled());
    }
}
