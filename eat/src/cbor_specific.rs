//! CBOR-specific definitions
//!
//! In order to support the submodules claim, the Entity Attestation Token (EAT) specification provides
//! a set of CBOR-specific definitions and a set of JSON-specific definitions.
//!
//! The CBOR-specific definitions from [Section 7.3.2](https://datatracker.ietf.org/doc/html/rfc9711#name-cbor-specific-cddl) are below.
//!
//! ```text
//! $EAT-CBOR-Tagged-Token /= CWT-Tagged-Message
//! $EAT-CBOR-Tagged-Token /= BUNDLE-Tagged-Message
//! CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token
//! JSON-Token-Inside-CBOR-Token = tstr
//! CBOR-Nested-Token = JSON-Token-Inside-CBOR-Token / CBOR-Token-Inside-CBOR-Token
//! Nested-Token = CBOR-Nested-Token
//! Submodule = Claims-Set / CBOR-Nested-Token /  Detached-Submodule-Digest
//! ```
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
//! These do not mesh well with the approach of using procedural macros to maintain one structure for JSON use
//! and one structure for CBOR use with TryFrom serving as a bridge (and the structure names distinguished by
//! presence/absence of Cbor suffix). The problem is that while the Submodule definitions share the same components,
//! they are split across different structures. In JSON, the Detached-Submodule-Digest appears in the JSON-Selector type.
//! In CBOR, the Detached-Submodule-Digest appears in the Submodule definition. To harmonize these definitions with the
//! approach taken in this library, the following CDDL is used for CBOR.
//!
//! ```text
//! CBOR-Selector = CBOR-Nested-Token / Detached-Submodule-Digest
//! Submodule = Claims-Set / CBOR-Selector
//! ```
//! To adhere to the naming conventions in this library, JSON-Selector is represented by the Selector enum
//! and CBOR-Selector is represented by the SelectorCbor enum.
//!
//! | CDDL | Rust |
//! |------|------|
//! | `CBOR-Selector` | [`SelectorCbor`] |
//! | `Submodule` (CBOR) | [`SubmoduleCbor`] |
//! | `submods map` | [`SubmodsMapCbor`] |
//! | `Nested-Token` (CBOR) | [`super::arrays::NestedTokenCbor`] |
//! | `Wrapped-Claims-Set` (CBOR) | [`super::arrays::WrappedClaimsSetCbor`] |
//!

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use base64::{Engine, engine::general_purpose::STANDARD};
use ciborium::{ser::into_writer, value::Value};
use serde::{Deserialize, Serialize};

use crate::{
    arrays::DetachedSubmoduleDigestCbor,
    json_specific::{JsonSelectorValue, SubmodsMap, Submodule},
    maps::ClaimsSetClaimsCbor,
};

// EAT-CBOR-Token = $EAT-CBOR-Tagged-Token / $EAT-CBOR-Untagged-Token
// $EAT-CBOR-Untagged-Token /= CWT-Untagged-Message
// $EAT-CBOR-Untagged-Token /= BUNDLE-Untagged-Message

/// Represents options available for encoding Submodule claims using CBOR except Claims-Set.
///
/// Submodule support for CBOR-encoded EATs relies on the following CDDL definitions:
///
/// ```text
/// $EAT-CBOR-Tagged-Token /= CWT-Tagged-Message
/// $EAT-CBOR-Tagged-Token /= BUNDLE-Tagged-Message
/// CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token
/// JSON-Token-Inside-CBOR-Token = tstr
/// CBOR-Nested-Token = JSON-Token-Inside-CBOR-Token / CBOR-Token-Inside-CBOR-Token
/// Nested-Token = CBOR-Nested-Token
/// Submodule = Claims-Set / CBOR-Nested-Token /  Detached-Submodule-Digest
/// ```
/// As noted in the [module documentation](./index.html), the CBOR-specific and JSON-specific definitions do not
/// align well with the naming practices of this library. The following definition is used in lieu
/// of Nested-Token when defining Submodule for CBOR-encoded EATs.
/// ```text
/// CBOR-Selector = CBOR-Nested-Token / Detached-Submodule-Digest
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SelectorCbor {
    JsonTokenInsideCborToken(String),
    CborTokenInsideCborToken(Vec<u8>),
    DetachedSubmoduleDigest(DetachedSubmoduleDigestCbor),
}

/// Represents the options available for encoding Submodule claims using CBOR.
///
/// EAT defines Submodule as below for CBOR-encoded tokens.
/// ```text
/// Submodule = Claims-Set / CBOR-Nested-Token / Detached-Submodule-Digest
/// ```
/// As noted in the [module documentation](./index.html), a modified definition is used in this library
/// to better align with naming and structure management practices.
/// ```text
/// Submodule = Claims-Set / CBOR-Selector
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SubmoduleCbor {
    ClaimsSet(Box<ClaimsSetClaimsCbor>),
    SelectorCbor(SelectorCbor),
}
impl TryFrom<Value> for SubmoduleCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match &value {
            Value::Map(_) => {
                // Claims-Set is a CBOR map; serialize the Value then deserialize as ClaimsSetClaimsCbor
                let mut buf = vec![];
                into_writer(&value, &mut buf)
                    .map_err(|e| format!("Failed to serialize Value for ClaimsSet: {e}"))?;
                let cs: ClaimsSetClaimsCbor = ciborium::de::from_reader(buf.as_slice())
                    .map_err(|e| format!("Failed to deserialize ClaimsSet: {e}"))?;
                Ok(SubmoduleCbor::ClaimsSet(Box::new(cs)))
            }
            Value::Text(s) => {
                // JSON-Token-Inside-CBOR-Token = tstr
                Ok(SubmoduleCbor::SelectorCbor(
                    SelectorCbor::JsonTokenInsideCborToken(s.clone()),
                ))
            }
            Value::Bytes(b) => {
                // CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token
                Ok(SubmoduleCbor::SelectorCbor(
                    SelectorCbor::CborTokenInsideCborToken(b.clone()),
                ))
            }
            Value::Array(_) => {
                // Detached-Submodule-Digest is an array
                let mut buf = vec![];
                into_writer(&value, &mut buf).map_err(|e| {
                    format!("Failed to serialize Value for DetachedSubmoduleDigest: {e}")
                })?;
                let dsd: DetachedSubmoduleDigestCbor = ciborium::de::from_reader(buf.as_slice())
                    .map_err(|e| format!("Failed to deserialize DetachedSubmoduleDigest: {e}"))?;
                Ok(SubmoduleCbor::SelectorCbor(
                    SelectorCbor::DetachedSubmoduleDigest(dsd),
                ))
            }
            _ => Err(format!(
                "Unexpected CBOR type for SubmoduleCbor: {:?}",
                value
            )),
        }
    }
}
impl TryFrom<&Value> for SubmoduleCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}
impl TryFrom<Submodule> for SubmoduleCbor {
    type Error = String;
    fn try_from(value: Submodule) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
impl TryFrom<&Submodule> for SubmoduleCbor {
    type Error = String;
    fn try_from(value: &Submodule) -> Result<Self, Self::Error> {
        match value {
            Submodule::ClaimsSet(b) => {
                let cs_cbor: ClaimsSetClaimsCbor = (&**b).try_into()?;
                Ok(SubmoduleCbor::ClaimsSet(Box::new(cs_cbor)))
            }
            Submodule::JsonSelector(js) => match &js.nested_token {
                JsonSelectorValue::JwtMessage(v) => Ok(SubmoduleCbor::SelectorCbor(
                    SelectorCbor::JsonTokenInsideCborToken(v.clone()),
                )),
                JsonSelectorValue::CborTokenInsideJsonToken(v) => {
                    let b = STANDARD
                        .decode(v)
                        .map_err(|e| format!("Failed to decode base64: {e}"))?;
                    Ok(SubmoduleCbor::SelectorCbor(
                        SelectorCbor::CborTokenInsideCborToken(b),
                    ))
                }
                JsonSelectorValue::DetachedEatBundle(deb) => {
                    let mut encoded_token = vec![];
                    into_writer(deb, &mut encoded_token)
                        .map_err(|e| format!("Failed to serialize DetachedEatBundle: {e}"))?;
                    Ok(SubmoduleCbor::SelectorCbor(
                        SelectorCbor::CborTokenInsideCborToken(encoded_token),
                    ))
                }
                JsonSelectorValue::DetachedSubmoduleDigest(v) => Ok(SubmoduleCbor::SelectorCbor(
                    SelectorCbor::DetachedSubmoduleDigest(v.try_into()?),
                )),
            },
        }
    }
}

/// CBOR encoding/decoding of the submods map: `{ + text => Submodule }`.
///
/// RFC 9711 Section 4.2.18 defines the submodules claim as a map of named submodules:
/// ```text
/// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmodsMapCbor(pub BTreeMap<String, SubmoduleCbor>);

impl TryFrom<Value> for SubmodsMapCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match &value {
            Value::Map(entries) => {
                let mut map = BTreeMap::new();
                for (k, v) in entries {
                    let key = match k.as_text() {
                        Some(s) => s.to_string(),
                        None => return Err(format!("submods map key must be text, got: {:?}", k)),
                    };
                    let submod = SubmoduleCbor::try_from(v.clone())?;
                    map.insert(key, submod);
                }
                Ok(SubmodsMapCbor(map))
            }
            _ => Err(format!("Expected CBOR map for submods, got: {:?}", value)),
        }
    }
}
impl TryFrom<&Value> for SubmodsMapCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}
impl TryFrom<SubmodsMap> for SubmodsMapCbor {
    type Error = String;
    fn try_from(value: SubmodsMap) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
impl TryFrom<&SubmodsMap> for SubmodsMapCbor {
    type Error = String;
    fn try_from(value: &SubmodsMap) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::new();
        for (k, v) in &value.0 {
            let submod_cbor = SubmoduleCbor::try_from(v)?;
            map.insert(k.clone(), submod_cbor);
        }
        Ok(SubmodsMapCbor(map))
    }
}
