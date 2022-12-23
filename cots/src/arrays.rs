//! Array-based structs from the Concise Trust Anchor Store (CoTS) spec
use alloc::format;
use alloc::string::{String, ToString};
use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use alloc::{vec, vec::Vec};
use serde::ser::Error as OtherError;

use crate::choices::*;
use crate::maps::*;

use cbor_derive::StructToArray;

// concise-ta-stores = [+ concise-ta-store-map]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentGroupList(pub Vec<EnvironmentGroupListMap>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentGroupListCbor(pub Vec<EnvironmentGroupListMapCbor>);

// todo closure error handling
impl TryFrom<&Value> for EnvironmentGroupListCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => Ok(EnvironmentGroupListCbor(
                v.iter()
                    .map(|m| EnvironmentGroupListMapCbor::try_from(m).unwrap())
                    .collect(),
            )),
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
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TrustAnchor {
    pub format: PkixTaType,
    #[cbor(value = "Bytes")]
    pub data: Vec<u8>,
}
