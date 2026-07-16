//! Choice-based types from the Concise Trust Anchor Store (CoTS) spec
//! ([draft-ietf-rats-concise-ta-stores]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `$pkix-ta-type` | [`PkixTaType`] / [`PkixTaTypeKnown`] |
//! | `tas-list-purpose` | [`TasListPurpose`] |
//!
//! [draft-ietf-rats-concise-ta-stores]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-concise-ta-stores

use alloc::string::{String, ToString};

use ciborium::value::Value;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// $concise-tag-type-choice /= #6.999(bytes .cbor concise-ta-stores)

// $pkix-ta-type /= tastore.pkix-cert-type
// $pkix-ta-type /= tastore.pkix-tainfo-type
// $pkix-ta-type /= tastore.pkix-spki-type
// tastore.pkix-cert-type = 0
// tastore.pkix-tainfo-type = 1
// tastore.pkix-spki-type = 2
/// Represents the type of a PKIX trust anchor in a Concise TA Store.
///
/// Wraps the known variants defined by `$pkix-ta-type` in the CoTS specification.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PkixTaType {
    Known(PkixTaTypeKnown),
}

/// Known PKIX trust anchor type values: certificate, TrustAnchorInfo, or SubjectPublicKeyInfo.
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum PkixTaTypeKnown {
    Cert = 0,
    TaInfo = 1,
    Spki = 2,
}
impl TryFrom<Value> for PkixTaType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match PkixTaTypeKnown::try_from(vs) {
                    Ok(val) => Ok(PkixTaType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for PkixTaType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match PkixTaTypeKnown::try_from(vs) {
                    Ok(val) => Ok(PkixTaType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// $tas-list-purpose /= "cots"
// $tas-list-purpose /= "corim"
// $tas-list-purpose /= "comid"
// $tas-list-purpose /= "coswid"
// $tas-list-purpose /= "eat"
// $tas-list-purpose /= "key-attestation"
// $tas-list-purpose /= "certificate"
/// Represents the intended purpose of a trust anchor store list in the CoTS specification.
///
/// Defined by `$tas-list-purpose` with values such as "cots", "corim", "eat", etc.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum TasListPurpose {
    cots,
    corim,
    comid,
    coswid,
    eat,
    key_attestation,
    certificate,
}
impl TryFrom<Value> for TasListPurpose {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => match TasListPurpose::try_from(s) {
                Ok(val) => Ok(val),
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for TasListPurpose {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => match TasListPurpose::try_from(s) {
                Ok(val) => Ok(val),
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<String> for TasListPurpose {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cots" => Ok(TasListPurpose::cots),
            "corim" => Ok(TasListPurpose::corim),
            "comid" => Ok(TasListPurpose::comid),
            "coswid" => Ok(TasListPurpose::coswid),
            "eat" => Ok(TasListPurpose::eat),
            "key-attestation" => Ok(TasListPurpose::key_attestation),
            "certificate" => Ok(TasListPurpose::certificate),
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&String> for TasListPurpose {
    type Error = String;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cots" => Ok(TasListPurpose::cots),
            "corim" => Ok(TasListPurpose::corim),
            "comid" => Ok(TasListPurpose::comid),
            "coswid" => Ok(TasListPurpose::coswid),
            "eat" => Ok(TasListPurpose::eat),
            "key-attestation" => Ok(TasListPurpose::key_attestation),
            "certificate" => Ok(TasListPurpose::certificate),
            _ => Err("".to_string()),
        }
    }
}
