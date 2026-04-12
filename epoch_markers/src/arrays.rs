//! Array-based structs from the Epoch Markers spec ([draft-ietf-rats-epoch-markers-03]).
//!
//! | CDDL | Rust |
//! |------|------|
//! | `MessageImprint` | [`MessageImprint`] / [`MessageImprintCbor`] |
//! | `GeneralName` | [`GeneralName`] / [`GeneralNameCbor`] |
//! | `epoch-tick-list` | [`EpochTickList`] |
//!
//! [draft-ietf-rats-epoch-markers-03]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03

use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData};

use cbor_derive::StructToArray;
use ciborium::{cbor, value::Value};
use serde::{Deserialize, Serialize};
use serde::{
    de::{Error, Visitor},
    ser::Error as OtherError,
};

use crate::choices::EpochTick;

/// `MessageImprint` from the CBOR-encoded TSTInfo, see [Epoch Markers Section 4.1.3].
///
/// ```text
/// MessageImprint = [
///   hashAlg : int
///   hashValue : bstr
/// ]
/// ```
///
/// [Epoch Markers Section 4.1.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03#section-4.1.3
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MessageImprint {
    #[cbor(value = "Integer")]
    pub hash_alg: i64,
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub hash_value: Vec<u8>,
}

/// `GeneralName` from the CBOR-encoded TSTInfo, see [Epoch Markers Section 4.1.3].
///
/// ```text
/// GeneralName = [ GeneralNameType : int, GeneralNameValue : any ]
/// ```
///
/// See [RFC 5280 Section 4.2.1.6] for type/value definitions.
///
/// [Epoch Markers Section 4.1.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03#section-4.1.3
/// [RFC 5280 Section 4.2.1.6]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.6
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralName {
    /// GeneralName type indicator.
    pub name_type: i64,
    /// GeneralName value (type-dependent).
    pub name_value: Value,
}

/// CBOR-encoded counterpart of [`GeneralName`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralNameCbor {
    /// GeneralName type indicator.
    pub name_type: i64,
    /// GeneralName value (type-dependent).
    pub name_value: Value,
}

impl TryFrom<Value> for GeneralNameCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(ref arr) if arr.len() == 2 => {
                let name_type: i64 = arr[0]
                    .as_integer()
                    .ok_or("expected integer for GeneralNameType")?
                    .try_into()
                    .map_err(|_| "GeneralNameType out of range".to_string())?;
                Ok(Self {
                    name_type,
                    name_value: arr[1].clone(),
                })
            }
            _ => Err("expected 2-element array for GeneralName".to_string()),
        }
    }
}

impl TryFrom<&Value> for GeneralNameCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

impl TryFrom<&GeneralName> for GeneralNameCbor {
    type Error = String;
    fn try_from(value: &GeneralName) -> Result<Self, Self::Error> {
        Ok(Self {
            name_type: value.name_type,
            name_value: value.name_value.clone(),
        })
    }
}

impl TryFrom<GeneralName> for GeneralNameCbor {
    type Error = String;
    fn try_from(value: GeneralName) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&GeneralNameCbor> for GeneralName {
    type Error = String;
    fn try_from(value: &GeneralNameCbor) -> Result<Self, Self::Error> {
        Ok(Self {
            name_type: value.name_type,
            name_value: value.name_value.clone(),
        })
    }
}

impl TryFrom<GeneralNameCbor> for GeneralName {
    type Error = String;
    fn try_from(value: GeneralNameCbor) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// `epoch-tick-list = [ + epoch-tick ]`.
///
/// A sequence of tick values used one at a time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochTickList(pub Vec<EpochTick>);
