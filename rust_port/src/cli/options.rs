//! Command-line options parsing and validation
//!
//! This module defines the CLI arguments for SharpHound using clap's derive API,
//! matching the original C# CommandLineParser options.

use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;

/// SharpHound - Active Directory Reconnaissance Tool
///
/// Collects Active Directory data for analysis with BloodHound.
/// This tool is for AUTHORIZED security testing only.
#[derive(Parser, Debug, Clone)]
#[command(name = "SharpHound")]
#[command(version = "2.8.0")]
#[command(about = "Active Directory enumeration for BloodHound", long_about = None)]
pub struct Options {
    // === Collection Method Options ===
    /// Collection methods to use (comma-separated)
    ///
    /// Available methods: Group, LocalGroup, LocalAdmin, RDP, DCOM, PSRemote,
    /// Session, Trusts, ACL, Container, ComputerOnly, GPOLocalGroup, LoggedOn,
    /// ObjectProps, SPNTargets, UserRights, Default, DCOnly, CARegistry,
    /// DCRegistry, CertServices, WebClientService, LdapServices, SmbInfo,
    /// NTLMRegistry, All
    #[arg(short = 'c', long, default_value = "Default", value_delimiter = ',')]
    pub collectionmethods: Vec<String>,

    // === Domain/Forest Options ===
    /// Specify domain to enumerate
    #[arg(short = 'd', long)]
    pub domain: Option<String>,

    /// Search all available domains in the forest
    #[arg(short = 's', long)]
    pub searchforest: bool,

    /// Recurse domain trusts to search
    #[arg(long)]
    pub recursedomains: bool,

    /// Stealth collection (reduces network noise - prefer DCOnly whenever possible!)
    #[arg(long)]
    pub stealth: bool,

    // === LDAP Query Options ===
    /// Add an LDAP filter to the pregenerated filter
    #[arg(short = 'f', long)]
    pub ldapfilter: Option<String>,

    /// Base Distinguished Name to start the LDAP search at
    #[arg(long)]
    pub distinguishedname: Option<String>,

    /// Path to file containing computer names to enumerate
    #[arg(long)]
    pub computerfile: Option<PathBuf>,

    /// Partition LDAP queries into smaller chunks to reduce server load
    #[arg(long)]
    pub partitionldapqueries: bool,

    // === Output Options ===
    /// Directory to output files to
    #[arg(long, default_value = ".")]
    pub outputdirectory: PathBuf,

    /// String to prepend to output file names
    #[arg(long)]
    pub outputprefix: Option<String>,

    /// Filename for cache (defaults to machine-specific identifier)
    #[arg(long)]
    pub cachename: Option<String>,

    /// Keep cache in memory and don't write to disk
    #[arg(long)]
    pub memcache: bool,

    /// Rebuild cache and remove all entries
    #[arg(long)]
    pub rebuildcache: bool,

    /// Use random filenames for output
    #[arg(long)]
    pub randomfilenames: bool,

    /// Filename for the ZIP archive
    #[arg(long)]
    pub zipfilename: Option<String>,

    /// Don't create ZIP archive of output files
    #[arg(long)]
    pub nozip: bool,

    /// Password protect the ZIP archive
    #[arg(long)]
    pub zippassword: Option<String>,

    /// Add CSV tracking requests to computers
    #[arg(long)]
    pub trackcomputercalls: bool,

    /// Pretty-print JSON output
    #[arg(long)]
    pub prettyprint: bool,

    // === LDAP Connection Options ===
    /// Username for LDAP authentication
    #[arg(long)]
    pub ldapusername: Option<String>,

    /// Password for LDAP authentication
    #[arg(long)]
    pub ldappassword: Option<String>,

    /// Override domain controller to pull LDAP from (can result in data loss)
    #[arg(long)]
    pub domaincontroller: Option<String>,

    /// Override port for LDAP
    #[arg(long, default_value = "0")]
    pub ldapport: u16,

    /// Override port for LDAPS
    #[arg(long, default_value = "0")]
    pub ldapsslport: u16,

    /// Only connect to LDAP SSL, disallowing fallback
    #[arg(long)]
    pub forcesecureldap: bool,

    /// Disable certificate verification when using LDAPS
    #[arg(long)]
    pub disablecertverification: bool,

    /// Disable Kerberos signing/sealing
    #[arg(long)]
    pub disablesigning: bool,

    // === Session Enumeration Options ===
    /// Use local admin credentials for session enumeration instead of domain credentials
    #[arg(long)]
    pub dolocaladminsessionenum: bool,

    /// Username for local administrator (requires dolocaladminsessionenum)
    #[arg(long)]
    pub localadminusername: Option<String>,

    /// Password for local administrator (requires dolocaladminsessionenum)
    #[arg(long)]
    pub localadminpassword: Option<String>,

    // === Enumeration Behavior Options ===
    /// Skip checking if port 445 (SMB) is open
    #[arg(long)]
    pub skipportcheck: bool,

    /// Timeout for port checks in milliseconds
    #[arg(long, default_value = "10000")]
    pub portchecktimeout: u32,

    /// Skip password age check when enumerating computers
    #[arg(long)]
    pub skippasswordcheck: bool,

    /// Exclude domain controllers from session/localgroup enumeration (for ATA/ATP evasion)
    #[arg(long)]
    pub excludedcs: bool,

    /// Add delay after computer requests in milliseconds
    #[arg(long, default_value = "0")]
    pub throttle: u32,

    /// Add jitter to throttle (percentage)
    #[arg(long, default_value = "0")]
    pub jitter: u32,

    /// Number of threads to run enumeration with
    #[arg(short = 't', long, default_value = "50")]
    pub threads: usize,

    /// Skip registry-based logged-on user enumeration
    #[arg(long)]
    pub skipregistryloggedon: bool,

    /// Override the username to filter for NetSessionEnum
    #[arg(long)]
    pub overrideusername: Option<String>,

    /// Override DNS suffix for API calls
    #[arg(long)]
    pub realdnsname: Option<String>,

    /// Collect all LDAP properties from objects
    #[arg(long)]
    pub collectallproperties: bool,

    // === Loop Collection Options ===
    /// Enable loop collection mode
    #[arg(short = 'l', long)]
    pub loop_mode: bool,

    /// Loop duration (format: hh:mm:ss, default: 02:00:00)
    #[arg(long, value_parser = parse_duration)]
    pub loopduration: Option<Duration>,

    /// Delay between loops (format: hh:mm:ss)
    #[arg(long, value_parser = parse_duration)]
    pub loopinterval: Option<Duration>,

    // === Misc Options ===
    /// Interval to display status in milliseconds
    #[arg(long, default_value = "30000")]
    pub statusinterval: u32,

    /// Verbosity level (lower is more verbose: 0=Trace, 1=Debug, 2=Information, 3=Warning, 4=Error)
    #[arg(short = 'v', long, default_value = "2")]
    pub verbosity: u8,
}

/// Parse duration string in format hh:mm:ss
///
/// Examples: "05:00:00" → 5 hours, "00:03:00" → 3 minutes
fn parse_duration(s: &str) -> Result<Duration, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(format!("Duration must be in format hh:mm:ss, got: {}", s));
    }

    let hours: u64 = parts[0].parse().map_err(|_| format!("Invalid hours: {}", parts[0]))?;
    let minutes: u64 = parts[1].parse().map_err(|_| format!("Invalid minutes: {}", parts[1]))?;
    let seconds: u64 = parts[2].parse().map_err(|_| format!("Invalid seconds: {}", parts[2]))?;

    if minutes >= 60 {
        return Err(format!("Minutes must be < 60, got: {}", minutes));
    }
    if seconds >= 60 {
        return Err(format!("Seconds must be < 60, got: {}", seconds));
    }

    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    Ok(Duration::from_secs(total_seconds))
}

impl Options {
    /// Validate options for consistency and requirements
    ///
    /// Returns `Err` if validation fails with a descriptive error message
    pub fn validate(&self) -> Result<(), String> {
        // Check LDAP username/password pair
        if self.ldapusername.is_some() != self.ldappassword.is_some() {
            return Err("You must specify both LDAPUsername and LDAPPassword if using these options".to_string());
        }

        // Check local admin session enum options
        if self.dolocaladminsessionenum {
            if self.localadminusername.is_none() || self.localadminpassword.is_none() {
                return Err("You must specify both LocalAdminUsername and LocalAdminPassword if using --dolocaladminsessionenum".to_string());
            }
        }

        if (self.localadminusername.is_some() || self.localadminpassword.is_some()) && !self.dolocaladminsessionenum {
            return Err("You must use --dolocaladminsessionenum in combination with --localadminusername and --localadminpassword".to_string());
        }

        // Validate jitter percentage
        if self.jitter > 100 {
            return Err(format!("Jitter must be <= 100%, got: {}%", self.jitter));
        }

        // Validate verbosity range
        if self.verbosity > 5 {
            return Err(format!("Verbosity must be 0-5, got: {}", self.verbosity));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_valid() {
        assert_eq!(parse_duration("05:00:00").unwrap(), Duration::from_secs(5 * 3600));
        assert_eq!(parse_duration("00:03:00").unwrap(), Duration::from_secs(3 * 60));
        assert_eq!(parse_duration("00:00:30").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_duration("02:30:15").unwrap(), Duration::from_secs(2 * 3600 + 30 * 60 + 15));
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert!(parse_duration("5:00").is_err());
        assert!(parse_duration("05:60:00").is_err()); // Minutes >= 60
        assert!(parse_duration("00:00:60").is_err()); // Seconds >= 60
        assert!(parse_duration("invalid").is_err());
        assert!(parse_duration("").is_err());
    }

    #[test]
    fn test_options_validate_ldap_credentials() {
        let mut opts = Options::parse_from(&["sharphound"]);
        assert!(opts.validate().is_ok());

        opts.ldapusername = Some("user".to_string());
        // Missing password
        assert!(opts.validate().is_err());

        opts.ldappassword = Some("pass".to_string());
        assert!(opts.validate().is_ok());
    }

    #[test]
    fn test_options_validate_localadmin_session_enum() {
        let mut opts = Options::parse_from(&["sharphound"]);

        opts.dolocaladminsessionenum = true;
        // Missing username/password
        assert!(opts.validate().is_err());

        opts.localadminusername = Some("admin".to_string());
        opts.localadminpassword = Some("password".to_string());
        assert!(opts.validate().is_ok());
    }

    #[test]
    fn test_options_validate_localadmin_without_flag() {
        let mut opts = Options::parse_from(&["sharphound"]);
        opts.localadminusername = Some("admin".to_string());
        opts.localadminpassword = Some("password".to_string());

        // dolocaladminsessionenum not set
        assert!(opts.validate().is_err());
    }

    #[test]
    fn test_options_validate_jitter() {
        let mut opts = Options::parse_from(&["sharphound"]);
        opts.jitter = 25;
        assert!(opts.validate().is_ok());

        opts.jitter = 101;
        assert!(opts.validate().is_err());
    }

    #[test]
    fn test_options_validate_verbosity() {
        let mut opts = Options::parse_from(&["sharphound"]);
        opts.verbosity = 2;
        assert!(opts.validate().is_ok());

        opts.verbosity = 6;
        assert!(opts.validate().is_err());
    }

    #[test]
    fn test_options_parse_collection_methods() {
        let opts = Options::parse_from(&["sharphound", "-c", "Group,Session,ACL"]);
        assert_eq!(opts.collectionmethods, vec!["Group", "Session", "ACL"]);
    }

    #[test]
    fn test_options_default_values() {
        let opts = Options::parse_from(&["sharphound"]);

        assert_eq!(opts.collectionmethods, vec!["Default"]);
        assert_eq!(opts.threads, 50);
        assert_eq!(opts.verbosity, 2);
        assert_eq!(opts.statusinterval, 30000);
        assert_eq!(opts.portchecktimeout, 10000);
        assert!(!opts.stealth);
        assert!(!opts.nozip);
    }

    #[test]
    fn test_options_parse_domain() {
        let opts = Options::parse_from(&["sharphound", "-d", "example.com"]);
        assert_eq!(opts.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_options_parse_flags() {
        let opts = Options::parse_from(&[
            "sharphound",
            "--stealth",
            "--memcache",
            "--nozip",
            "--prettyprint",
        ]);

        assert!(opts.stealth);
        assert!(opts.memcache);
        assert!(opts.nozip);
        assert!(opts.prettyprint);
    }
}
