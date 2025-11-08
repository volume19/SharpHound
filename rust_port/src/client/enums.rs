//! Collection method enums and types
//!
//! This module defines the collection methods available in SharpHound,
//! representing the different types of Active Directory data that can be enumerated.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Collection method options representing the possible collection methods
/// specified in command-line options.
///
/// These options are parsed from the command line and later resolved to
/// the bitflag-based `CollectionMethod` type for internal use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CollectionMethodOptions {
    /// No collection methods
    None,
    /// Group membership enumeration
    Group,
    /// Active session enumeration
    Session,
    /// Logged on user enumeration
    LoggedOn,
    /// Domain trust enumeration
    Trusts,
    /// Access Control List (ACL) enumeration
    ACL,
    /// LDAP object properties collection
    ObjectProps,
    /// Remote Desktop Users group enumeration
    RDP,
    /// Distributed COM Users group enumeration
    DCOM,
    /// Local Administrator group enumeration
    LocalAdmin,
    /// PSRemote group enumeration
    PSRemote,
    /// Service Principal Name (SPN) targets
    SPNTargets,
    /// Container hierarchy enumeration
    Container,
    /// GPO-based local group enumeration
    GPOLocalGroup,
    /// Local group enumeration
    LocalGroup,
    /// User rights assignment enumeration
    UserRights,
    /// Default collection methods (standard set)
    Default,
    /// Domain Controller only collection (LDAP-focused)
    DCOnly,
    /// Computer-focused collection methods only
    ComputerOnly,
    /// Certificate Authority registry settings
    CARegistry,
    /// Domain Controller registry settings
    DCRegistry,
    /// Certificate Services (ADCS) enumeration
    CertServices,
    /// WebClient service enumeration
    WebClientService,
    /// LDAP service enumeration
    LdapServices,
    /// SMB information gathering
    SmbInfo,
    /// NTLM registry settings
    NTLMRegistry,
    /// All available collection methods
    All,
}

impl fmt::Display for CollectionMethodOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "None",
            Self::Group => "Group",
            Self::Session => "Session",
            Self::LoggedOn => "LoggedOn",
            Self::Trusts => "Trusts",
            Self::ACL => "ACL",
            Self::ObjectProps => "ObjectProps",
            Self::RDP => "RDP",
            Self::DCOM => "DCOM",
            Self::LocalAdmin => "LocalAdmin",
            Self::PSRemote => "PSRemote",
            Self::SPNTargets => "SPNTargets",
            Self::Container => "Container",
            Self::GPOLocalGroup => "GPOLocalGroup",
            Self::LocalGroup => "LocalGroup",
            Self::UserRights => "UserRights",
            Self::Default => "Default",
            Self::DCOnly => "DCOnly",
            Self::ComputerOnly => "ComputerOnly",
            Self::CARegistry => "CARegistry",
            Self::DCRegistry => "DCRegistry",
            Self::CertServices => "CertServices",
            Self::WebClientService => "WebClientService",
            Self::LdapServices => "LdapServices",
            Self::SmbInfo => "SmbInfo",
            Self::NTLMRegistry => "NTLMRegistry",
            Self::All => "All",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for CollectionMethodOptions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "group" => Ok(Self::Group),
            "session" => Ok(Self::Session),
            "loggedon" => Ok(Self::LoggedOn),
            "trusts" => Ok(Self::Trusts),
            "acl" => Ok(Self::ACL),
            "objectprops" => Ok(Self::ObjectProps),
            "rdp" => Ok(Self::RDP),
            "dcom" => Ok(Self::DCOM),
            "localadmin" => Ok(Self::LocalAdmin),
            "psremote" => Ok(Self::PSRemote),
            "spntargets" => Ok(Self::SPNTargets),
            "container" => Ok(Self::Container),
            "gpolocalgroup" => Ok(Self::GPOLocalGroup),
            "localgroup" => Ok(Self::LocalGroup),
            "userrights" => Ok(Self::UserRights),
            "default" => Ok(Self::Default),
            "dconly" => Ok(Self::DCOnly),
            "computeronly" => Ok(Self::ComputerOnly),
            "caregistry" => Ok(Self::CARegistry),
            "dcregistry" => Ok(Self::DCRegistry),
            "certservices" => Ok(Self::CertServices),
            "webclientservice" => Ok(Self::WebClientService),
            "ldapservices" => Ok(Self::LdapServices),
            "smbinfo" => Ok(Self::SmbInfo),
            "ntlmregistry" => Ok(Self::NTLMRegistry),
            "all" => Ok(Self::All),
            _ => Err(format!("Invalid collection method: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_method_from_str_case_insensitive() {
        assert_eq!("Group".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::Group);
        assert_eq!("group".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::Group);
        assert_eq!("GROUP".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::Group);
    }

    #[test]
    fn test_collection_method_from_str_all_variants() {
        assert_eq!("None".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::None);
        assert_eq!("All".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::All);
        assert_eq!("DCOnly".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::DCOnly);
        assert_eq!("LoggedOn".parse::<CollectionMethodOptions>().unwrap(), CollectionMethodOptions::LoggedOn);
    }

    #[test]
    fn test_collection_method_from_str_invalid() {
        assert!("InvalidMethod".parse::<CollectionMethodOptions>().is_err());
        assert!("".parse::<CollectionMethodOptions>().is_err());
    }

    #[test]
    fn test_collection_method_display() {
        assert_eq!(format!("{}", CollectionMethodOptions::Group), "Group");
        assert_eq!(format!("{}", CollectionMethodOptions::All), "All");
        assert_eq!(format!("{}", CollectionMethodOptions::DCOnly), "DCOnly");
    }

    #[test]
    fn test_collection_method_roundtrip() {
        for method in [
            CollectionMethodOptions::None,
            CollectionMethodOptions::Group,
            CollectionMethodOptions::Session,
            CollectionMethodOptions::All,
            CollectionMethodOptions::DCOnly,
        ] {
            let s = format!("{}", method);
            let parsed: CollectionMethodOptions = s.parse().unwrap();
            assert_eq!(parsed, method);
        }
    }

    #[test]
    fn test_collection_method_serialize_deserialize() {
        let method = CollectionMethodOptions::Group;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, r#""Group""#);

        let deserialized: CollectionMethodOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, method);
    }
}
