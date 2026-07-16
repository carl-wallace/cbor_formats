//! JSON-specific definitions
//!
//! In order to support the submodules claim, the Entity Attestation Token (EAT) specification provides
//! a set of CBOR-specific definitions and a set of JSON-specific definitions.
//!
//! The JSON-specific definitions from [Section 7.3.3](https://datatracker.ietf.org/doc/html/rfc9711#name-json-specific-cddl) are below.
//!
//! ```text
//! $JSON-Selector-Value /= JWT-Message / CBOR-Token-Inside-JSON-Token / Detached-EAT-Bundle / Detached-Submodule-Digest
//!
//! JSON-Selector = [
//!    type : $JSON-Selector-Type,
//!    nested-token : $JSON-Selector-Value
//! ]
//! Submodule = Claims-Set / JSON-Selector
//! ```
//!
//! This module provides support for JSON-encoded Submodule claims. See [cbor_specific](../cbor_specific/index.html) module for
//! details regarding support for CBOR-encoded Submodule claims.
//!
//! | CDDL | Rust |
//! |------|------|
//! | `$JSON-Selector-Type` | [`JsonSelectorType`] |
//! | `$JSON-Selector-Value` | [`JsonSelectorValue`] |
//! | `JSON-Selector` | [`JsonSelector`] |
//! | `$JSON-Selector-Value` (for-deb variant) | [`JsonSelectorForDebValue`] |
//! | `Selector-For-Deb` | [`SelectorForDeb`] |
//! | `Submodule` (JSON) | [`Submodule`] |

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::ops::Deref;

use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};

use crate::{
    arrays::{DetachedEatBundle, DetachedSubmoduleDigest},
    cbor_specific::{SelectorCbor, SubmodsMapCbor, SubmoduleCbor},
    maps::{ClaimsSetClaims, ClaimsSetClaimsCbor},
};

// EAT-JSON-Token = $EAT-JSON-Token-Formats
//
// $EAT-JSON-Token-Formats /= JWT-Message
// $EAT-JSON-Token-Formats /= BUNDLE-Untagged-Message
//
//
// Nested-Token = JSON-Selector

/// Represents values used to indicate type of nested token in JSON-Selector as defined in [EAT Section 4.2.18].
/// Note, while this enum is extensible the related [JsonSelectorValue] type is not, at present.
///
/// ```text
/// $JSON-Selector-Type /= "JWT" / "CBOR" / "BUNDLE" / "DIGEST"
/// ```
///
/// [EAT Section 4.2.18]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.18
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde_enum_str::Deserialize_enum_str,
    serde_enum_str::Serialize_enum_str,
)]
#[allow(missing_docs)]
pub enum JsonSelectorType {
    #[serde(rename = "JWT")]
    Jwt,
    #[serde(rename = "CBOR")]
    Cbor,
    #[serde(rename = "BUNDLE")]
    Bundle,
    #[serde(rename = "DIGEST")]
    Digest,
    #[serde(other)]
    Other(String),
}

// todo make JsonSelectorValue and JsonSelector extensible
/// Represents values used to indicate type of nested token in JSON-Selector as defined in [EAT Section 4.2.18]
///
/// ```text
/// $JSON-Selector-Value /= JWT-Message /
///                   CBOR-Token-Inside-JSON-Token /
///                   Detached-EAT-Bundle /
///                   Detached-Submodule-Digest
/// ```
///
/// [EAT Section 4.2.18]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.18
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum JsonSelectorValue {
    JwtMessage(String),
    CborTokenInsideJsonToken(String),
    DetachedEatBundle(DetachedEatBundle),
    DetachedSubmoduleDigest(DetachedSubmoduleDigest),
}
/// Deserialize a `JsonSelectorValue` given the `token_type` discriminator.
///
/// This avoids the fragile dot-counting heuristic for distinguishing JWT from
/// base64-encoded CBOR tokens — the `token_type` field tells us unambiguously.
fn deserialize_selector_value(
    token_type: &JsonSelectorType,
    value: serde_json::Value,
) -> Result<JsonSelectorValue, String> {
    match token_type {
        JsonSelectorType::Jwt => match value {
            serde_json::Value::String(s) => Ok(JsonSelectorValue::JwtMessage(s)),
            _ => Err("JWT selector value must be a string".to_string()),
        },
        JsonSelectorType::Cbor => match value {
            serde_json::Value::String(s) => Ok(JsonSelectorValue::CborTokenInsideJsonToken(s)),
            _ => Err("CBOR selector value must be a base64-encoded string".to_string()),
        },
        JsonSelectorType::Bundle => serde_json::from_value::<DetachedEatBundle>(value)
            .map(JsonSelectorValue::DetachedEatBundle)
            .map_err(|e| format!("Failed to parse DetachedEatBundle: {e}")),
        JsonSelectorType::Digest => serde_json::from_value::<DetachedSubmoduleDigest>(value)
            .map(JsonSelectorValue::DetachedSubmoduleDigest)
            .map_err(|e| format!("Failed to parse DetachedSubmoduleDigest: {e}")),
        JsonSelectorType::Other(t) => Err(format!("Unknown JSON-Selector-Type: {t}")),
    }
}

/// Provides token_type and nested_token for JSON-encoded submodules
///
/// The `JSON-Selector` array is defined in [EAT Section 4.2.18] and represents a token type and a
/// nested token values suitable for use in representing a JSON-encoded submodule claim.
///
/// ```text
/// JSON-Selector = [
///    type : $JSON-Selector-Type,
///    nested-token : $JSON-Selector-Value
/// ]
/// ```
/// [SelectorForDeb] is used for DetachedEATBundles.
///
/// [EAT Section 4.2.18]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.18
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct JsonSelector {
    pub token_type: JsonSelectorType,
    pub nested_token: JsonSelectorValue,
}
impl Serialize for JsonSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.token_type)?;
        seq.serialize_element(&self.nested_token)?;
        seq.end()
    }
}
impl<'de> Deserialize<'de> for JsonSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr = <Vec<serde_json::Value>>::deserialize(deserializer)?;
        if arr.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "JSON-Selector must be a 2-element array, got {}",
                arr.len()
            )));
        }
        let token_type: JsonSelectorType =
            serde_json::from_value(arr[0].clone()).map_err(serde::de::Error::custom)?;
        let nested_token = deserialize_selector_value(&token_type, arr[1].clone())
            .map_err(serde::de::Error::custom)?;

        Ok(JsonSelector {
            token_type,
            nested_token,
        })
    }
}

/// Represents JSON-Selector values for use within a Detached-EAT-Bundle per RFC 9711.
///
/// Unlike [JsonSelectorValue], this excludes the `DetachedEatBundle` variant since a DEB
/// cannot contain another DEB.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum JsonSelectorForDebValue {
    JwtMessage(String),
    CborTokenInsideJsonToken(String),
    DetachedSubmoduleDigest(DetachedSubmoduleDigest),
}
/// Deserialize a `JsonSelectorForDebValue` given the `token_type` discriminator.
///
/// Like [`deserialize_selector_value`] but excludes the `DetachedEatBundle` variant
/// since a DEB cannot contain another DEB.
fn deserialize_selector_for_deb_value(
    token_type: &JsonSelectorType,
    value: serde_json::Value,
) -> Result<JsonSelectorForDebValue, String> {
    match token_type {
        JsonSelectorType::Jwt => match value {
            serde_json::Value::String(s) => Ok(JsonSelectorForDebValue::JwtMessage(s)),
            _ => Err("JWT selector value must be a string".to_string()),
        },
        JsonSelectorType::Cbor => match value {
            serde_json::Value::String(s) => {
                Ok(JsonSelectorForDebValue::CborTokenInsideJsonToken(s))
            }
            _ => Err("CBOR selector value must be a base64-encoded string".to_string()),
        },
        JsonSelectorType::Digest => serde_json::from_value::<DetachedSubmoduleDigest>(value)
            .map(JsonSelectorForDebValue::DetachedSubmoduleDigest)
            .map_err(|e| format!("Failed to parse DetachedSubmoduleDigest: {e}")),
        JsonSelectorType::Bundle => {
            Err("BUNDLE selector type is not permitted inside a Detached-EAT-Bundle".to_string())
        }
        JsonSelectorType::Other(t) => Err(format!("Unknown JSON-Selector-Type: {t}")),
    }
}

/// JSON-Selector variant for use within a Detached-EAT-Bundle per RFC 9711.
///
/// Similar to [JsonSelector] but uses [JsonSelectorForDebValue] to exclude the
/// `DetachedEatBundle` option.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct SelectorForDeb {
    pub token_type: JsonSelectorType,
    pub nested_token: JsonSelectorForDebValue,
}
impl Serialize for SelectorForDeb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.token_type)?;
        seq.serialize_element(&self.nested_token)?;
        seq.end()
    }
}
impl<'de> Deserialize<'de> for SelectorForDeb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr = <Vec<serde_json::Value>>::deserialize(deserializer)?;
        if arr.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "Selector-For-Deb must be a 2-element array, got {}",
                arr.len()
            )));
        }
        let token_type: JsonSelectorType =
            serde_json::from_value(arr[0].clone()).map_err(serde::de::Error::custom)?;
        let nested_token = deserialize_selector_for_deb_value(&token_type, arr[1].clone())
            .map_err(serde::de::Error::custom)?;

        Ok(SelectorForDeb {
            token_type,
            nested_token,
        })
    }
}

// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
//
// Submodule = Claims-Set / JSON-Selector

/// Represents a JSON-encoded EAT Submodule as defined in RFC 9711 Section 4.2.18.
///
/// A Submodule is either a nested Claims-Set or a JSON-Selector. Use [SubmoduleCbor]
/// for CBOR-encoded EATs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Submodule {
    //todo consider changing ClaimsSetClaims to be an enum with ClaimsSet accepting duplicates
    ClaimsSet(Box<ClaimsSetClaims>),
    JsonSelector(JsonSelector),
}
impl TryFrom<SubmoduleCbor> for Submodule {
    type Error = String;
    fn try_from(value: SubmoduleCbor) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
impl TryFrom<&SubmoduleCbor> for Submodule {
    type Error = String;
    fn try_from(value: &SubmoduleCbor) -> Result<Self, Self::Error> {
        match value {
            SubmoduleCbor::ClaimsSet(b) => {
                let cs: &ClaimsSetClaimsCbor = b.deref();
                let cs_json: ClaimsSetClaims = cs.try_into()?;
                Ok(Submodule::ClaimsSet(Box::new(cs_json)))
            }
            SubmoduleCbor::SelectorCbor(SelectorCbor::CborTokenInsideCborToken(b)) => {
                let js = JsonSelector {
                    token_type: JsonSelectorType::Cbor,
                    nested_token: JsonSelectorValue::CborTokenInsideJsonToken(STANDARD.encode(b)),
                };
                Ok(Submodule::JsonSelector(js))
            }
            SubmoduleCbor::SelectorCbor(SelectorCbor::JsonTokenInsideCborToken(s)) => {
                let js = JsonSelector {
                    token_type: JsonSelectorType::Jwt,
                    nested_token: JsonSelectorValue::JwtMessage(s.clone()),
                };
                Ok(Submodule::JsonSelector(js))
            }
            SubmoduleCbor::SelectorCbor(SelectorCbor::DetachedSubmoduleDigest(dsm)) => {
                let js = JsonSelector {
                    token_type: JsonSelectorType::Digest,
                    nested_token: JsonSelectorValue::DetachedSubmoduleDigest(dsm.try_into()?),
                };
                Ok(Submodule::JsonSelector(js))
            }
        }
    }
}

/// JSON encoding/decoding of the submods map: `{ + text => Submodule }`.
///
/// RFC 9711 Section 4.2.18 defines the submodules claim as a map of named submodules:
/// ```text
/// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
/// ```
/// Use [SubmodsMapCbor] for CBOR-encoded EATs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmodsMap(pub BTreeMap<String, Submodule>);

impl TryFrom<SubmodsMapCbor> for SubmodsMap {
    type Error = String;
    fn try_from(value: SubmodsMapCbor) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
impl TryFrom<&SubmodsMapCbor> for SubmodsMap {
    type Error = String;
    fn try_from(value: &SubmodsMapCbor) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::new();
        for (k, v) in &value.0 {
            let submod = Submodule::try_from(v)?;
            map.insert(k.clone(), submod);
        }
        Ok(SubmodsMap(map))
    }
}
