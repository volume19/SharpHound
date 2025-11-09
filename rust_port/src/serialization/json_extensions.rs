//! JSON serialization extensions for SharpHound
//!
//! This module provides custom serialization/deserialization logic for SharpHound types.
//! In the C# version, this used Newtonsoft.Json with custom contract resolvers and converters.
//! In Rust, we use serde which provides similar functionality through derive macros and custom
//! serializers/deserializers.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Label enum for BloodHound object types
///
/// This is a placeholder for the Label enum from SharpHoundCommonLib.Enums.
/// The full implementation will be provided when SharpHoundCommonLib is ported.
///
/// Labels represent the type of Active Directory object (User, Computer, Group, etc.)
/// and are used throughout BloodHound for categorizing nodes in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Label {
    /// Base/Unknown label (default fallback)
    Base,
    /// User object
    User,
    /// Computer object
    Computer,
    /// Group object
    Group,
    /// Domain object
    Domain,
    /// Organizational Unit
    OU,
    /// GPO (Group Policy Object)
    GPO,
    /// Container object
    Container,
    /// Local Group
    LocalGroup,
    /// Local User
    LocalUser,
    /// ADCS (Active Directory Certificate Services)
    ADCS,
    /// Certificate Template
    CertTemplate,
    /// Enterprise CA
    EnterpriseCA,
    /// NTAuth Store
    NTAuthStore,
    /// Root CA
    RootCA,
    /// AIACA (Authority Information Access CA)
    AIACA,
    /// IssuancePolicy
    IssuancePolicy,
}

impl Default for Label {
    fn default() -> Self {
        Label::Base
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Label::Base => "Base",
            Label::User => "User",
            Label::Computer => "Computer",
            Label::Group => "Group",
            Label::Domain => "Domain",
            Label::OU => "OU",
            Label::GPO => "GPO",
            Label::Container => "Container",
            Label::LocalGroup => "LocalGroup",
            Label::LocalUser => "LocalUser",
            Label::ADCS => "ADCS",
            Label::CertTemplate => "CertTemplate",
            Label::EnterpriseCA => "EnterpriseCA",
            Label::NTAuthStore => "NTAuthStore",
            Label::RootCA => "RootCA",
            Label::AIACA => "AIACA",
            Label::IssuancePolicy => "IssuancePolicy",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Label {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Base" => Ok(Label::Base),
            "User" => Ok(Label::User),
            "Computer" => Ok(Label::Computer),
            "Group" => Ok(Label::Group),
            "Domain" => Ok(Label::Domain),
            "OU" => Ok(Label::OU),
            "GPO" => Ok(Label::GPO),
            "Container" => Ok(Label::Container),
            "LocalGroup" => Ok(Label::LocalGroup),
            "LocalUser" => Ok(Label::LocalUser),
            "ADCS" => Ok(Label::ADCS),
            "CertTemplate" => Ok(Label::CertTemplate),
            "EnterpriseCA" => Ok(Label::EnterpriseCA),
            "NTAuthStore" => Ok(Label::NTAuthStore),
            "RootCA" => Ok(Label::RootCA),
            "AIACA" => Ok(Label::AIACA),
            "IssuancePolicy" => Ok(Label::IssuancePolicy),
            _ => Err(()),
        }
    }
}

/// Custom serializer for Label enum
///
/// This replicates the behavior of KindConvertor in JsonExtensions.cs.
/// Serializes Label as a string (e.g., "User", "Computer").
impl Serialize for Label {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Custom deserializer for Label enum
///
/// This replicates the behavior of KindConvertor in JsonExtensions.cs.
/// Deserializes from string, falling back to Label::Base if parsing fails.
impl<'de> Deserialize<'de> for Label {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Label::from_str(&s).unwrap_or(Label::Base))
    }
}

/// Cache serialization settings
///
/// In C#, CacheContractResolver was used to enable serialization of properties
/// with private setters. In Rust with serde, this is not needed because:
/// 1. Serde automatically handles private fields when using derive macros
/// 2. Field visibility is controlled by the module system, not accessors
/// 3. #[serde(skip)] can be used to exclude specific fields
///
/// This struct provides helper methods for cache-related serialization settings.
pub struct CacheSerializerSettings;

impl CacheSerializerSettings {
    /// Serialize an object to JSON string with cache-friendly settings
    ///
    /// Uses serde_json with pretty printing disabled for compact output.
    pub fn to_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }

    /// Deserialize an object from JSON string with cache-friendly settings
    ///
    /// Allows missing fields and provides default values where applicable.
    pub fn from_json<'a, T: Deserialize<'a>>(json: &'a str) -> Result<T, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_display() {
        assert_eq!(Label::User.to_string(), "User");
        assert_eq!(Label::Computer.to_string(), "Computer");
        assert_eq!(Label::Group.to_string(), "Group");
        assert_eq!(Label::Base.to_string(), "Base");
    }

    #[test]
    fn test_label_from_str_valid() {
        assert_eq!(Label::from_str("User").unwrap(), Label::User);
        assert_eq!(Label::from_str("Computer").unwrap(), Label::Computer);
        assert_eq!(Label::from_str("Group").unwrap(), Label::Group);
        assert_eq!(Label::from_str("Domain").unwrap(), Label::Domain);
    }

    #[test]
    fn test_label_from_str_invalid() {
        // Invalid strings should return Err
        assert!(Label::from_str("InvalidLabel").is_err());
        assert!(Label::from_str("").is_err());
        assert!(Label::from_str("user").is_err()); // Case-sensitive
    }

    #[test]
    fn test_label_default() {
        assert_eq!(Label::default(), Label::Base);
    }

    #[test]
    fn test_label_serialize() {
        let label = Label::User;
        let json = serde_json::to_string(&label).unwrap();
        assert_eq!(json, "\"User\"");
    }

    #[test]
    fn test_label_deserialize_valid() {
        let json = "\"Computer\"";
        let label: Label = serde_json::from_str(json).unwrap();
        assert_eq!(label, Label::Computer);
    }

    #[test]
    fn test_label_deserialize_invalid_fallback() {
        // Invalid label should fall back to Base (matching C# KindConvertor behavior)
        let json = "\"InvalidLabel\"";
        let label: Label = serde_json::from_str(json).unwrap();
        assert_eq!(label, Label::Base);
    }

    #[test]
    fn test_label_roundtrip() {
        let original = Label::Group;
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Label = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_cache_serializer_settings_to_json() {
        #[derive(Serialize)]
        struct TestStruct {
            name: String,
            value: i32,
        }

        let test = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        let json = CacheSerializerSettings::to_json(&test).unwrap();
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"test\""));
        assert!(json.contains("\"value\""));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_cache_serializer_settings_from_json() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct TestStruct {
            name: String,
            value: i32,
        }

        let json = r#"{"name":"test","value":42}"#;
        let result: TestStruct = CacheSerializerSettings::from_json(json).unwrap();

        assert_eq!(result.name, "test");
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_all_label_variants() {
        // Ensure all variants can be serialized and deserialized
        let labels = vec![
            Label::Base,
            Label::User,
            Label::Computer,
            Label::Group,
            Label::Domain,
            Label::OU,
            Label::GPO,
            Label::Container,
            Label::LocalGroup,
            Label::LocalUser,
            Label::ADCS,
            Label::CertTemplate,
            Label::EnterpriseCA,
            Label::NTAuthStore,
            Label::RootCA,
            Label::AIACA,
            Label::IssuancePolicy,
        ];

        for label in labels {
            let json = serde_json::to_string(&label).unwrap();
            let deserialized: Label = serde_json::from_str(&json).unwrap();
            assert_eq!(label, deserialized);
        }
    }
}
