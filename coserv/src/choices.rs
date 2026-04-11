//! Choice-based types from the CoSERV specification ([draft-ietf-rats-coserv-05]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `comid.oid-type / ~uri` | [`CoservProfile`] / [`CoservProfileCbor`] |
//! | `$artifact-type` | [`ArtifactType`] |
//! | `$result-type` | [`ResultType`] |
//!
//! [draft-ietf-rats-coserv-05]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05

use ciborium::value::Value;

use alloc::string::{String, ToString};
use common::OidType;
use serde::{Deserialize, Serialize};

use num_enum::TryFromPrimitive;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

// profile = comid.oid-type / ~uri
//
// where oid-type = bytes and ~uri means the content of #6.32(tstr), i.e. tstr.

/// The CoSERV `profile` choice: `comid.oid-type / ~uri`.
///
/// - `Oid` — an OID encoded as raw bytes (`oid-type = bytes`)
/// - `Uri` — a URI as a plain text string (`~uri`, i.e. unwrapped `#6.32(tstr)`)
///
/// See [CoSERV Section 4](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CoservProfile {
    Uri(String),
    Oid(OidType),
}

/// CBOR-encoded form of [`CoservProfile`].
pub type CoservProfileCbor = CoservProfile;

impl TryFrom<Value> for CoservProfile {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(b) => Ok(Self::Oid(OidType(b))),
            Value::Text(s) => Ok(Self::Uri(s)),
            _ => Err("Expected bytes or text for CoservProfile".to_string()),
        }
    }
}

impl TryFrom<&Value> for CoservProfile {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(b) => Ok(Self::Oid(OidType(b.clone()))),
            Value::Text(s) => Ok(Self::Uri(s.clone())),
            _ => Err("Expected bytes or text for CoservProfile".to_string()),
        }
    }
}

impl TryFrom<&CoservProfile> for Value {
    type Error = String;
    fn try_from(value: &CoservProfile) -> Result<Self, Self::Error> {
        match value {
            CoservProfile::Oid(OidType(b)) => Ok(Value::Bytes(b.clone())),
            CoservProfile::Uri(s) => Ok(Value::Text(s.clone())),
        }
    }
}

// artifact-type = &(
//   endorsed-values: 0
//   trust-anchors: 1
//   reference-values: 2
// )

/// Represents the type of artifact being queried or returned in a CoSERV exchange.
///
/// Defined by the `artifact-type` enumeration in [CoSERV Section 4.3.1].
///
/// [CoSERV Section 4.3.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.3.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[repr(i64)]
#[allow(missing_docs)]
pub enum ArtifactType {
    EndorsedValues = 0,
    TrustAnchors = 1,
    ReferenceValues = 2,
}

impl TryFrom<Value> for ArtifactType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match ArtifactType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("Failed to parse ArtifactType".to_string()),
                },
                Err(_) => Err("Failed to parse ArtifactType integer".to_string()),
            },
            _ => Err("Expected Integer for ArtifactType".to_string()),
        }
    }
}
impl TryFrom<&Value> for ArtifactType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match ArtifactType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("Failed to parse ArtifactType".to_string()),
                },
                Err(_) => Err("Failed to parse ArtifactType integer".to_string()),
            },
            _ => Err("Expected Integer for ArtifactType".to_string()),
        }
    }
}

// result-type = &(
//   collected-artifacts: 0
//   source-artifacts: 1
//   both: 2
// )

/// Represents the desired result format in a CoSERV query.
///
/// Defined by the `result-type` enumeration in [CoSERV Section 4.3.3].
///
/// [CoSERV Section 4.3.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.3.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[repr(i64)]
#[allow(missing_docs)]
pub enum ResultType {
    CollectedArtifacts = 0,
    SourceArtifacts = 1,
    Both = 2,
}

impl TryFrom<Value> for ResultType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match ResultType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("Failed to parse ResultType".to_string()),
                },
                Err(_) => Err("Failed to parse ResultType integer".to_string()),
            },
            _ => Err("Expected Integer for ResultType".to_string()),
        }
    }
}
impl TryFrom<&Value> for ResultType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match ResultType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("Failed to parse ResultType".to_string()),
                },
                Err(_) => Err("Failed to parse ResultType integer".to_string()),
            },
            _ => Err("Expected Integer for ResultType".to_string()),
        }
    }
}
