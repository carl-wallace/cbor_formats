//! Choice-based structs from the Concise Software Identification Tags (CoSWID) spec

use ciborium::value::Value;
use serde::{Deserialize, Serialize};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::maps::{EvidenceEntry, PayloadEntry};
use common::IntType;
use num_enum::TryFromPrimitive;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

// payload-or-evidence //= ( payload => payload-entry )
// payload-or-evidence //= ( evidence => evidence-entry )
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PayloadOrEvidence {
    Payload(PayloadEntry),
    Evidence(EvidenceEntry),
}

/// label = text / int
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Label {
    Text(String),
    Integer(IntType),
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OneOrMoreRole {
    One(Role),
    More(Vec<Role>),
}

//todo closure error handling
impl TryFrom<Value> for OneOrMoreRole {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => Ok(OneOrMoreRole::More(
                v.iter().map(|m| Role::try_from(m).unwrap()).collect(),
            )),
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
            Value::Array(v) => Ok(OneOrMoreRole::More(
                v.iter().map(|m| Role::try_from(m).unwrap()).collect(),
            )),
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
// shared=1
// private=2
// abandon=3
//
// $ownership /= shared
// $ownership /= private
// $ownership /= abandon
// $ownership /= int / text
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Ownership {
    Known(OwnershipKnown),
    Text(String),
    IntExtensions(i64),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum OwnershipKnown {
    Shared = 1,
    Private = 2,
    Abandon = 3,
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Rel {
    Known(RelKnown),
    Text(String),
    IntExtensions(i64),
}

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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum UseChoice {
    Known(UseChoiceKnown),
    Text(String),
    IntExtensions(i64),
}

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
