//! Choice-based types from the Concise Software Identification Tags (CoSWID) spec ([RFC 9393]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `payload-or-evidence` | [`PayloadOrEvidence`] |
//! | `$role` | [`Role`] / [`RoleKnown`] |
//! | `one-or-more<role>` | [`OneOrMoreRole`] |
//! | `$ownership` | [`Ownership`] / [`OwnershipKnown`] |
//! | `$rel` | [`Rel`] / [`RelKnown`] |
//! | `$use-choice` | [`UseChoice`] / [`UseChoiceKnown`] |
//!
//! [RFC 9393]: https://datatracker.ietf.org/doc/html/rfc9393

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use ciborium::value::Value;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::maps::{EvidenceEntry, PayloadEntry};

// payload-or-evidence //= ( payload => payload-entry )
// payload-or-evidence //= ( evidence => evidence-entry )
/// Represents the CoSWID `payload-or-evidence` choice, which carries either a payload entry or an evidence entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PayloadOrEvidence {
    Payload(PayloadEntry),
    Evidence(EvidenceEntry),
}

// version scheme is defined in the corim crate

/// tag-creator=1
/// software-creator=2
/// aggregator=3
/// distributor=4
/// licensor=5
/// maintainer=6
/// $role /= tag-creator
/// $role /= software-creator
/// $role /= aggregator
/// $role /= distributor
/// $role /= licensor
/// $role /= maintainer
/// $role /= int / text
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Role {
    Known(RoleKnown),
    Text(String),
    IntExtensions(i64),
}

/// Well-known integer values for the CoSWID `$role` choice as defined in RFC 9393 Section 2.6.
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum RoleKnown {
    TagCreator = 1,
    SoftwareCreator = 2,
    Aggregator = 3,
    Distributor = 4,
    Licensor = 5,
    Maintainer = 6,
}

impl TryFrom<Value> for Role {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match RoleKnown::try_from(vs) {
                    Ok(val) => Ok(Role::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for Role {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match RoleKnown::try_from(vs) {
                    Ok(val) => Ok(Role::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

/// Represents one or more CoSWID `$role` values, supporting both singular and array forms.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OneOrMoreRole {
    One(Role),
    More(Vec<Role>),
}

impl TryFrom<Value> for OneOrMoreRole {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => {
                let roles: Result<Vec<_>, _> = v.iter().map(Role::try_from).collect();
                Ok(OneOrMoreRole::More(roles?))
            }
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match RoleKnown::try_from(vs) {
                    Ok(val) => Ok(OneOrMoreRole::One(Role::Known(val))),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for OneOrMoreRole {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => {
                let roles: Result<Vec<_>, _> = v.iter().map(Role::try_from).collect();
                Ok(OneOrMoreRole::More(roles?))
            }
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match RoleKnown::try_from(vs) {
                    Ok(val) => Ok(OneOrMoreRole::One(Role::Known(val))),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// ; ownership indices
// abandon=1
// private=2
// shared=3
//
// $ownership /= abandon
// $ownership /= private
// $ownership /= shared
// $ownership /= int / text
/// Represents the CoSWID `$ownership` choice indicating software ownership status (e.g., abandon, private, shared).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Ownership {
    Known(OwnershipKnown),
    Text(String),
    IntExtensions(i64),
}

/// Well-known integer values for the CoSWID `$ownership` choice as defined in RFC 9393 Section 2.7.
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum OwnershipKnown {
    Abandon = 1,
    Private = 2,
    Shared = 3,
}

impl TryFrom<Value> for Ownership {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match OwnershipKnown::try_from(vs) {
                    Ok(val) => Ok(Ownership::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for Ownership {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match OwnershipKnown::try_from(vs) {
                    Ok(val) => Ok(Ownership::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// ; rel indices
// ancestor=1
// component=2
// feature=3
// installationmedia=4
// packageinstaller=5
// parent=6
// patches=7
// requires=8
// see-also=9
// supersedes=10
// ; supplemental=11
//
// $rel /= ancestor
// $rel /= component
// $rel /= feature
// $rel /= installationmedia
// $rel /= packageinstaller
// $rel /= parent
// $rel /= patches
// $rel /= requires
// $rel /= see-also
// $rel /= supersedes
// $rel /= supplemental
// $rel /= -256..64436 / text
/// Represents the CoSWID `$rel` choice describing the relationship type in a link entry.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Rel {
    Known(RelKnown),
    Text(String),
    IntExtensions(i64),
}

/// Well-known integer values for the CoSWID `$rel` choice as defined in RFC 9393 Section 2.7.
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum RelKnown {
    Ancestor = 1,
    Component = 2,
    Feature = 3,
    InstallationMedia = 4,
    PackageInstaller = 5,
    Parent = 6,
    Patches = 7,
    Requires = 8,
    SeeAlso = 9,
    Supersedes = 10,
    Supplemental = 11,
}

impl TryFrom<Value> for Rel {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match RelKnown::try_from(vs) {
                    Ok(val) => Ok(Rel::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for Rel {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match RelKnown::try_from(vs) {
                    Ok(val) => Ok(Rel::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// ; use integer indices
// optional=1
// required=2
// recommended=3
//
// $use /= optional
// $use /= required
// $use /= recommended
// $use /= int / text
/// Represents the CoSWID `$use` choice indicating whether a link target is optional, required, or recommended.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum UseChoice {
    Known(UseChoiceKnown),
    Text(String),
    IntExtensions(i64),
}

/// Well-known integer values for the CoSWID `$use` choice as defined in RFC 9393 Section 2.7.
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum UseChoiceKnown {
    Optional = 1,
    Required = 2,
    Recommended = 3,
}

impl TryFrom<Value> for UseChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match UseChoiceKnown::try_from(vs) {
                    Ok(val) => Ok(UseChoice::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for UseChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match UseChoiceKnown::try_from(vs) {
                    Ok(val) => Ok(UseChoice::Known(val)),
                    Err(_) => Ok(Self::IntExtensions(vs)),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
