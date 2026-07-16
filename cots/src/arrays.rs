//! Array-based structs from the Concise Trust Anchor Store (CoTS) spec
//! ([draft-ietf-rats-concise-ta-stores]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `concise-ta-stores = [+ concise-ta-store-map]` | [`ConciseTaStores`] / [`ConciseTaStoresCbor`] |
//! | `environment-group-list` | [`EnvironmentGroupList`] / [`EnvironmentGroupListCbor`] |
//! | `trust-anchor` | [`TrustAnchor`] / [`TrustAnchorCbor`] |
//!
//! [draft-ietf-rats-concise-ta-stores]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-concise-ta-stores

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{fmt, marker::PhantomData};

use ciborium::{cbor, value::Value};
use serde::{Deserialize, Serialize, de::Error, de::Visitor, ser::Error as OtherError};

use cbor_derive::StructToArray;

use crate::{choices::*, maps::*};

// concise-ta-stores = [+ concise-ta-store-map]

/// JSON encoding/decoding of `concise-ta-stores`, a list of Concise TA Store maps.
///
/// Use [ConciseTaStoresCbor] for CBOR-encoded Concise TA Stores.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConciseTaStores(pub Vec<ConciseTaStoreMap>);

impl TryFrom<ConciseTaStoresCbor> for ConciseTaStores {
    type Error = String;
    fn try_from(value: ConciseTaStoresCbor) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<ConciseTaStoreMap>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}
impl TryFrom<&ConciseTaStoresCbor> for ConciseTaStores {
    type Error = String;
    fn try_from(value: &ConciseTaStoresCbor) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<ConciseTaStoreMap>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}

/// CBOR encoding/decoding of `concise-ta-stores`, a list of Concise TA Store maps.
///
/// Use [ConciseTaStores] for JSON-encoded Concise TA Stores.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConciseTaStoresCbor(pub Vec<ConciseTaStoreMapCbor>);

impl TryFrom<ConciseTaStores> for ConciseTaStoresCbor {
    type Error = String;
    fn try_from(value: ConciseTaStores) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<ConciseTaStoreMapCbor>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}
impl TryFrom<&ConciseTaStores> for ConciseTaStoresCbor {
    type Error = String;
    fn try_from(value: &ConciseTaStores) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<ConciseTaStoreMapCbor>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}

// environment-group-list = [* environment-group-list-map]

/// JSON encoding/decoding of `environment-group-list` from the CoTS specification.
///
/// Use [EnvironmentGroupListCbor] for CBOR-encoded Concise TA Stores.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentGroupList(pub Vec<EnvironmentGroupListMap>);

/// CBOR encoding/decoding of `environment-group-list` from the CoTS specification.
///
/// Use [EnvironmentGroupList] for JSON-encoded Concise TA Stores.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentGroupListCbor(pub Vec<EnvironmentGroupListMapCbor>);

impl TryFrom<&Value> for EnvironmentGroupListCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => {
                let items: Result<Vec<_>, _> = v
                    .iter()
                    .map(EnvironmentGroupListMapCbor::try_from)
                    .collect();
                Ok(EnvironmentGroupListCbor(items?))
            }
            _ => Err("Failed to parse value as an array for EnvironmentGroupListCbor".to_string()),
        }
    }
}

#[allow(unused_variables)]
impl TryFrom<&EnvironmentGroupList> for EnvironmentGroupListCbor {
    type Error = String;
    fn try_from(value: &EnvironmentGroupList) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<EnvironmentGroupListMapCbor>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}
#[allow(unused_variables)]
impl TryFrom<&EnvironmentGroupListCbor> for EnvironmentGroupList {
    type Error = String;
    fn try_from(value: &EnvironmentGroupListCbor) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<EnvironmentGroupListMap>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}

// trust-anchor = [
//   format => $pkix-ta-type
//   data => bstr
// ]

/// Represents a `trust-anchor` array from the CoTS specification.
///
/// Contains a PKIX trust anchor format indicator and the raw trust anchor data.
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TrustAnchor {
    pub format: PkixTaType,
    #[cbor(value = "Bytes")]
    pub data: Vec<u8>,
}
