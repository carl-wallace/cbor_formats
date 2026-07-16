//! Discovery document types from the CoSERV specification ([draft-ietf-rats-coserv-05 Section 6.1.1]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `coserv-well-known-info` | [`CoservWellKnownInfoMap`] / [`CoservWellKnownInfoMapCbor`] |
//! | `capability` | [`CapabilityMap`] / [`CapabilityMapCbor`] |
//! | `artifact-support` | `Vec<`[`ArtifactSupportType`]`>` |
//!
//! The discovery document is an independent structure, exempt from the
//! encoding rules in [Section 4.5](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.5).
//!
//! [draft-ietf-rats-coserv-05 Section 6.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-6.1.1

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{fmt, marker::PhantomData};

use ciborium::{cbor, value::Value};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

use cbor_derive::StructToMap;
use common::TupleCbor;
use cose::maps::{CoseKey, CoseKeyCbor, CoseKeySet};

/// Media type for CoSERV discovery documents in CBOR format.
pub const COSERV_DISCOVERY_CBOR_CONTENT_TYPE: &str = "application/coserv-discovery+cbor";

/// Media type for CoSERV discovery documents in JSON format.
pub const COSERV_DISCOVERY_JSON_CONTENT_TYPE: &str = "application/coserv-discovery+json";

// artifact-support = non-empty-array<[ ? "source", ? "collected" ]>

/// A member of the `artifact-support` array.
///
/// The `artifact-support` array contains one or both of `"source"` and
/// `"collected"`, indicating which artifact categories a capability supports.
///
/// See [CoSERV Section 6.1.1.1.2](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-6.1.1.1.2).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ArtifactSupportType {
    /// Source artifacts.
    #[serde(rename = "source")]
    Source,
    /// Collected artifacts.
    #[serde(rename = "collected")]
    Collected,
}

impl TryFrom<Value> for ArtifactSupportType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Value> for ArtifactSupportType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => match s.as_str() {
                "source" => Ok(Self::Source),
                "collected" => Ok(Self::Collected),
                _ => Err(format!("Unknown artifact support type: {s}")),
            },
            _ => Err("Expected text for ArtifactSupportType".to_string()),
        }
    }
}

// capability = {
//   media-type-label => cmw.media-type,
//   artifact-support-label => artifact-support
// }
//
// media-type-label = eat.JC<"media-type", 1>
// artifact-support-label = eat.JC<"artifact-support", 2>

/// A `capability` entry from [CoSERV Section 6.1.1.1.2].
///
/// ```text
/// capability = {
///   media-type-label => cmw.media-type,
///   artifact-support-label => artifact-support
/// }
/// ```
///
/// [CoSERV Section 6.1.1.1.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-6.1.1.1.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CapabilityMap {
    #[serde(rename = "media-type")]
    #[cbor(tag = "1", value = "Text")]
    pub media_type: String,
    #[serde(rename = "artifact-support")]
    #[cbor(tag = "2", value = "Array")]
    pub artifact_support: Vec<ArtifactSupportType>,
}

impl CapabilityMap {
    /// Validates that `artifact_support` is non-empty, per the CDDL
    /// `non-empty-array` constraint.
    pub fn validate(&self) -> Result<(), String> {
        if self.artifact_support.is_empty() {
            return Err("artifact-support must be non-empty".to_string());
        }
        Ok(())
    }
}

// coserv-well-known-info = {
//   version-label => version,
//   capabilities-label => [ + capability ],
//   api-endpoints-label => { + tstr => tstr },
//   ? result-verification-key-label => eat.JC<jwk.JWK_Set, cose.COSE_KeySet>
// }
//
// version-label = eat.JC<"version", 1>
// capabilities-label = eat.JC<"capabilities", 2>
// api-endpoints-label = eat.JC<"api-endpoints", 3>
// result-verification-key-label = eat.JC<"result-verification-key", 4>

/// The `coserv-well-known-info` discovery document from [CoSERV Section 6.1.1].
///
/// ```text
/// coserv-well-known-info = {
///   version-label => version,
///   capabilities-label => [ + capability ],
///   api-endpoints-label => { + tstr => tstr },
///   ? result-verification-key-label => eat.JC<jwk.JWK_Set, cose.COSE_KeySet>
/// }
/// ```
///
/// Note: the `result_verification_key` field uses [`CoseKeySet`] for both JSON
/// and CBOR forms. The CDDL specifies `jwk.JWK_Set` for JSON encoding, which
/// is not currently modeled.
///
/// [CoSERV Section 6.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-6.1.1
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoservWellKnownInfoMap {
    pub version: String,
    pub capabilities: Vec<CapabilityMap>,
    #[serde(rename = "api-endpoints")]
    pub api_endpoints: BTreeMap<String, String>,
    #[serde(
        rename = "result-verification-key",
        skip_serializing_if = "Option::is_none"
    )]
    pub result_verification_key: Option<CoseKeySet>,
}

impl CoservWellKnownInfoMap {
    /// Validates that:
    /// - `capabilities` is non-empty (`[+ capability]`)
    /// - `api_endpoints` is non-empty (`{+ tstr => tstr}`)
    /// - each capability's `artifact_support` is non-empty
    pub fn validate(&self) -> Result<(), String> {
        if self.capabilities.is_empty() {
            return Err("capabilities must be non-empty".to_string());
        }
        if self.api_endpoints.is_empty() {
            return Err("api-endpoints must be non-empty".to_string());
        }
        for cap in &self.capabilities {
            cap.validate()?;
        }
        Ok(())
    }
}

/// CBOR-encoded form of [`CoservWellKnownInfoMap`].
///
/// Uses integer labels per the `eat.JC` pattern:
/// - `1` = version
/// - `2` = capabilities
/// - `3` = api-endpoints
/// - `4` = result-verification-key (optional)
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct CoservWellKnownInfoMapCbor {
    pub version: String,
    pub capabilities: Vec<CapabilityMapCbor>,
    pub api_endpoints: BTreeMap<String, String>,
    pub result_verification_key: Option<Vec<CoseKeyCbor>>,
}

impl TryFrom<&CoservWellKnownInfoMapCbor> for CoservWellKnownInfoMap {
    type Error = String;
    fn try_from(value: &CoservWellKnownInfoMapCbor) -> Result<Self, Self::Error> {
        Ok(CoservWellKnownInfoMap {
            version: value.version.clone(),
            capabilities: value
                .capabilities
                .iter()
                .map(CapabilityMap::try_from)
                .collect::<Result<_, _>>()?,
            api_endpoints: value.api_endpoints.clone(),
            result_verification_key: match &value.result_verification_key {
                Some(keys) => Some(CoseKeySet(
                    keys.iter()
                        .map(CoseKey::try_from)
                        .collect::<Result<_, _>>()?,
                )),
                None => None,
            },
        })
    }
}

impl TryFrom<&CoservWellKnownInfoMap> for CoservWellKnownInfoMapCbor {
    type Error = String;
    fn try_from(value: &CoservWellKnownInfoMap) -> Result<Self, Self::Error> {
        Ok(CoservWellKnownInfoMapCbor {
            version: value.version.clone(),
            capabilities: value
                .capabilities
                .iter()
                .map(CapabilityMapCbor::try_from)
                .collect::<Result<_, _>>()?,
            api_endpoints: value.api_endpoints.clone(),
            result_verification_key: match &value.result_verification_key {
                Some(ks) => Some(
                    ks.0.iter()
                        .map(CoseKeyCbor::try_from)
                        .collect::<Result<_, _>>()?,
                ),
                None => None,
            },
        })
    }
}

impl TryFrom<Vec<(Value, Value)>> for CoservWellKnownInfoMapCbor {
    type Error = String;
    fn try_from(value: Vec<(Value, Value)>) -> Result<Self, Self::Error> {
        let mut m: BTreeMap<i32, Value> = BTreeMap::new();
        for (k, v) in value {
            let index: i32 = k
                .as_integer()
                .and_then(|i| i.try_into().ok())
                .ok_or_else(|| "Expected integer key in coserv-well-known-info map".to_string())?;
            m.insert(index, v);
        }

        // version (label 1)
        let version = m
            .get(&1)
            .and_then(|v| v.as_text())
            .ok_or_else(|| "Missing or invalid version (label 1)".to_string())?
            .to_string();

        // capabilities (label 2)
        let capabilities = match m.get(&2) {
            Some(v) => match v.as_array() {
                Some(a) => a
                    .iter()
                    .map(|v| CapabilityMapCbor::try_from(v.clone()))
                    .collect::<Result<_, _>>()?,
                None => return Err("capabilities (label 2) must be an array".to_string()),
            },
            None => return Err("Missing capabilities (label 2)".to_string()),
        };

        // api-endpoints (label 3)
        let api_endpoints = match m.get(&3) {
            Some(v) => match v.as_map() {
                Some(pairs) => {
                    let mut ep = BTreeMap::new();
                    for (k, v) in pairs {
                        let key = k
                            .as_text()
                            .ok_or_else(|| "api-endpoints key must be text".to_string())?
                            .to_string();
                        let val = v
                            .as_text()
                            .ok_or_else(|| "api-endpoints value must be text".to_string())?
                            .to_string();
                        ep.insert(key, val);
                    }
                    ep
                }
                None => return Err("api-endpoints (label 3) must be a map".to_string()),
            },
            None => return Err("Missing api-endpoints (label 3)".to_string()),
        };

        // result-verification-key (label 4, optional)
        let result_verification_key = match m.get(&4) {
            Some(v) => match v.as_array() {
                Some(a) => Some(
                    a.iter()
                        .map(|v| CoseKeyCbor::try_from(v.clone()))
                        .collect::<Result<_, _>>()?,
                ),
                None => {
                    return Err("result-verification-key (label 4) must be an array".to_string());
                }
            },
            None => None,
        };

        Ok(CoservWellKnownInfoMapCbor {
            version,
            capabilities,
            api_endpoints,
            result_verification_key,
        })
    }
}

impl TryFrom<&CoservWellKnownInfoMapCbor> for Vec<(Value, Value)> {
    type Error = String;
    fn try_from(value: &CoservWellKnownInfoMapCbor) -> Result<Self, Self::Error> {
        let mut v: Vec<(Value, Value)> = Vec::new();

        // version (label 1)
        v.push((Value::Integer(1.into()), Value::Text(value.version.clone())));

        // capabilities (label 2)
        v.push((
            Value::Integer(2.into()),
            Value::serialized(&value.capabilities).map_err(|e| format!("{e:?}"))?,
        ));

        // api-endpoints (label 3)
        let ep_pairs: Vec<(Value, Value)> = value
            .api_endpoints
            .iter()
            .map(|(k, v)| (Value::Text(k.clone()), Value::Text(v.clone())))
            .collect();
        v.push((Value::Integer(3.into()), Value::Map(ep_pairs)));

        // result-verification-key (label 4, optional)
        if let Some(keys) = &value.result_verification_key {
            v.push((
                Value::Integer(4.into()),
                Value::serialized(keys).map_err(|e| format!("{e:?}"))?,
            ));
        }

        Ok(v)
    }
}

impl Serialize for CoservWellKnownInfoMapCbor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v: Vec<(Value, Value)> = self.try_into().map_err(S::Error::custom)?;
        Value::Map(v).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CoservWellKnownInfoMapCbor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor;
        impl<'de> Visitor<'de> for MapVisitor {
            type Value = Vec<(Value, Value)>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = Vec::with_capacity(map.size_hint().unwrap_or(0).min(16));
                while let Some(value) = map.next_entry()? {
                    values.push(value);
                }
                values.retain(|(_, v)| *v != Value::Null);
                Ok(values)
            }
        }
        let pairs = deserializer.deserialize_map(MapVisitor)?;
        CoservWellKnownInfoMapCbor::try_from(pairs).map_err(D::Error::custom)
    }
}
