//! General-purpose choice types

use ciborium::value::Value;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

use alloc::string::{String, ToString};

/// The `version-scheme` socket is defined in [CoRIM Section 3.1.4.1.5.3].
///
/// ```text
/// $version-scheme /= &(multipartnumeric: 1)
/// $version-scheme /= &(multipartnumeric-suffix: 2)
/// $version-scheme /= &(alphanumeric: 3)
/// $version-scheme /= &(decimal: 4)
/// $version-scheme /= &(semver: 16384)
/// $version-scheme /= int / text
/// ```
///
/// [CoRIM Section 3.1.4.1.5.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(untagged)]
pub enum VersionScheme {
    Known(VersionSchemeKnown),
    Text(String),
    IntExtensions(i64),
}
impl TryFrom<VersionSchemeCbor> for VersionScheme {
    type Error = String;
    fn try_from(value: VersionSchemeCbor) -> Result<Self, Self::Error> {
        match value {
            VersionSchemeCbor::Known(vs) => match vs.try_into() {
                Ok(v) => Ok(Self::Known(v)),
                Err(e) => Err(e),
            },
            VersionSchemeCbor::Text(vs) => Ok(Self::Text(vs)),
            VersionSchemeCbor::IntExtensions(vs) => Ok(Self::IntExtensions(vs)),
        }
    }
}

impl TryFrom<&VersionSchemeCbor> for VersionScheme {
    type Error = String;
    fn try_from(value: &VersionSchemeCbor) -> Result<Self, Self::Error> {
        match value {
            VersionSchemeCbor::Known(vs) => match vs.try_into() {
                Ok(v) => Ok(Self::Known(v)),
                Err(e) => Err(e),
            },
            VersionSchemeCbor::Text(vs) => Ok(Self::Text(vs.clone())),
            VersionSchemeCbor::IntExtensions(vs) => Ok(Self::IntExtensions(*vs)),
        }
    }
}

impl TryFrom<Value> for VersionScheme {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match VersionSchemeKnown::try_from(vs) {
                    Ok(val) => Ok(Self::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for VersionScheme {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match VersionSchemeKnown::try_from(vs) {
                    Ok(val) => Ok(Self::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum VersionSchemeKnown {
    Multipartnumeric = 1,
    MultipartnumericSuffix = 2,
    AlphaNumeric = 3,
    Decimal = 4,
    Semver = 16384,
}
impl TryFrom<VersionSchemeKnownCbor> for VersionSchemeKnown {
    type Error = String;
    fn try_from(value: VersionSchemeKnownCbor) -> Result<Self, Self::Error> {
        match value {
            VersionSchemeKnownCbor::AlphaNumeric => Ok(Self::AlphaNumeric),
            VersionSchemeKnownCbor::Multipartnumeric => Ok(Self::Multipartnumeric),
            VersionSchemeKnownCbor::MultipartnumericSuffix => Ok(Self::MultipartnumericSuffix),
            VersionSchemeKnownCbor::Decimal => Ok(Self::Decimal),
            VersionSchemeKnownCbor::Semver => Ok(Self::Semver),
        }
    }
}
impl TryFrom<&VersionSchemeKnownCbor> for VersionSchemeKnown {
    type Error = String;
    fn try_from(value: &VersionSchemeKnownCbor) -> Result<Self, Self::Error> {
        match value {
            VersionSchemeKnownCbor::AlphaNumeric => Ok(Self::AlphaNumeric),
            VersionSchemeKnownCbor::Multipartnumeric => Ok(Self::Multipartnumeric),
            VersionSchemeKnownCbor::MultipartnumericSuffix => Ok(Self::MultipartnumericSuffix),
            VersionSchemeKnownCbor::Decimal => Ok(Self::Decimal),
            VersionSchemeKnownCbor::Semver => Ok(Self::Semver),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(untagged)]
pub enum VersionSchemeCbor {
    Known(VersionSchemeKnownCbor),
    Text(String),
    IntExtensions(i64),
}
impl TryFrom<VersionScheme> for VersionSchemeCbor {
    type Error = String;
    fn try_from(value: VersionScheme) -> Result<Self, Self::Error> {
        match value {
            VersionScheme::Known(vs) => match vs.try_into() {
                Ok(v) => Ok(Self::Known(v)),
                Err(e) => Err(e),
            },
            VersionScheme::Text(vs) => Ok(Self::Text(vs)),
            VersionScheme::IntExtensions(vs) => Ok(Self::IntExtensions(vs)),
        }
    }
}
impl TryFrom<&VersionScheme> for VersionSchemeCbor {
    type Error = String;
    fn try_from(value: &VersionScheme) -> Result<Self, Self::Error> {
        match value {
            VersionScheme::Known(vs) => match vs.try_into() {
                Ok(v) => Ok(Self::Known(v)),
                Err(e) => Err(e),
            },
            VersionScheme::Text(vs) => Ok(VersionSchemeCbor::Text(vs.clone())),
            VersionScheme::IntExtensions(vs) => Ok(VersionSchemeCbor::IntExtensions(*vs)),
        }
    }
}
impl TryFrom<Value> for VersionSchemeCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match VersionSchemeKnownCbor::try_from(vs) {
                    Ok(val) => Ok(Self::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for VersionSchemeCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match VersionSchemeKnownCbor::try_from(vs) {
                    Ok(val) => Ok(Self::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum VersionSchemeKnownCbor {
    Multipartnumeric = 1,
    MultipartnumericSuffix = 2,
    AlphaNumeric = 3,
    Decimal = 4,
    Semver = 16384,
}
impl TryFrom<VersionSchemeKnown> for VersionSchemeKnownCbor {
    type Error = String;
    fn try_from(value: VersionSchemeKnown) -> Result<Self, Self::Error> {
        match value {
            VersionSchemeKnown::AlphaNumeric => Ok(Self::AlphaNumeric),
            VersionSchemeKnown::Multipartnumeric => Ok(Self::Multipartnumeric),
            VersionSchemeKnown::MultipartnumericSuffix => Ok(Self::MultipartnumericSuffix),
            VersionSchemeKnown::Decimal => Ok(Self::Decimal),
            VersionSchemeKnown::Semver => Ok(Self::Semver),
        }
    }
}
impl TryFrom<&VersionSchemeKnown> for VersionSchemeKnownCbor {
    type Error = String;
    fn try_from(value: &VersionSchemeKnown) -> Result<Self, Self::Error> {
        match value {
            VersionSchemeKnown::AlphaNumeric => Ok(Self::AlphaNumeric),
            VersionSchemeKnown::Multipartnumeric => Ok(Self::Multipartnumeric),
            VersionSchemeKnown::MultipartnumericSuffix => Ok(Self::MultipartnumericSuffix),
            VersionSchemeKnown::Decimal => Ok(Self::Decimal),
            VersionSchemeKnown::Semver => Ok(Self::Semver),
        }
    }
}
