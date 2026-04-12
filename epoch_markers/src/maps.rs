//! Map-based structs from the Epoch Markers spec ([draft-ietf-rats-epoch-markers-03]).
//!
//! | CDDL | Rust |
//! |------|------|
//! | `TST-info-based-on-CBOR-time-tag` | [`TstInfoCborTimeTag`] / [`TstInfoCborTimeTagCbor`] |
//! | `profiled-etime` | [`ProfiledEtime`] / [`ProfiledEtimeCbor`] |
//!
//! [draft-ietf-rats-epoch-markers-03]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData};

use cbor_derive::StructToMap;
use ciborium::tag::Required;
use ciborium::{cbor, value::Value};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

use common::OidType;
use common::tuple::Tuple;
#[allow(unused_imports)]
use common::tuple::TupleCbor;
use etime::choices::BaseTime;
use etime::maps::EtimeMap;

use crate::arrays::{GeneralName, GeneralNameCbor, MessageImprint, MessageImprintCbor};

/// Profiled etime for epoch markers: `profiled-etime = #6.1001(timeMap)`.
///
/// A newtype around [`EtimeMap`] that constrains the base time to key 1 (`~time`).
///
/// ```text
/// profiled-etime = #6.1001(timeMap)
/// timeMap = {
///   1 => ~time
///   ? -8 => profiled-duration
///   * int => any
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfiledEtime(pub EtimeMap);

impl ProfiledEtime {
    /// Validates that the profiled etime contains key 1 (`~time`) as its base time,
    /// not key 4 (`~decfrac`) or key 5 (`~bigfloat`).
    pub fn validate(&self) -> Result<(), String> {
        match &self.0.base_time {
            BaseTime::IntegerSecs(_) | BaseTime::FloatSecs(_) => Ok(()),
            BaseTime::DecFrac(_, _) => Err(
                "profiled-etime requires base time key 1 (~time), got key 4 (~decfrac)".to_string(),
            ),
            BaseTime::BigFloat(_, _) => Err(
                "profiled-etime requires base time key 1 (~time), got key 5 (~bigfloat)"
                    .to_string(),
            ),
        }
    }
}

/// CBOR-encoded form of [`ProfiledEtime`], wrapped in tag 1001.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfiledEtimeCbor(pub Required<EtimeMap, 1001>);

impl TryFrom<&Value> for ProfiledEtimeCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(1001, inner) => {
                // Serialize the inner Value to bytes, then deserialize as EtimeMap
                let mut buf = vec![];
                ciborium::ser::into_writer(inner.as_ref(), &mut buf)
                    .map_err(|e| format!("Failed to serialize etime inner value: {e}"))?;
                let map: EtimeMap = ciborium::de::from_reader(buf.as_slice())
                    .map_err(|e| format!("Failed to deserialize etime map: {e}"))?;
                Ok(Self(Required(map)))
            }
            _ => Err("expected tag 1001 for profiled-etime".to_string()),
        }
    }
}

impl TryFrom<Value> for ProfiledEtimeCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&ProfiledEtime> for ProfiledEtimeCbor {
    type Error = String;
    fn try_from(value: &ProfiledEtime) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.clone())))
    }
}

impl TryFrom<ProfiledEtime> for ProfiledEtimeCbor {
    type Error = String;
    fn try_from(value: ProfiledEtime) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0)))
    }
}

impl TryFrom<&ProfiledEtimeCbor> for ProfiledEtime {
    type Error = String;
    fn try_from(value: &ProfiledEtimeCbor) -> Result<Self, Self::Error> {
        Ok(Self(value.0.0.clone()))
    }
}

impl TryFrom<ProfiledEtimeCbor> for ProfiledEtime {
    type Error = String;
    fn try_from(value: ProfiledEtimeCbor) -> Result<Self, Self::Error> {
        Ok(Self(value.0.0))
    }
}

/// OID choice for the TST info policy field: `oid = #6.111(bstr) / #6.112(bstr)`.
///
/// Tag 111 is an absolute OID ([RFC 9090]), tag 112 is a relative OID.
///
/// [RFC 9090]: https://www.rfc-editor.org/rfc/rfc9090.html
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OidChoice {
    AbsoluteOid(Required<OidType, 111>),
    RelativeOid(Required<OidType, 112>),
}

impl<'de> Deserialize<'de> for OidChoice {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        Self::try_from(&value).map_err(Error::custom)
    }
}

impl TryFrom<&Value> for OidChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, inner) => match inner.as_bytes() {
                Some(b) => Ok(Self::AbsoluteOid(Required(OidType(b.clone())))),
                None => Err("expected bytes inside tag 111 for OID".to_string()),
            },
            Value::Tag(112, inner) => match inner.as_bytes() {
                Some(b) => Ok(Self::RelativeOid(Required(OidType(b.clone())))),
                None => Err("expected bytes inside tag 112 for relative OID".to_string()),
            },
            _ => Err("expected tag 111 or 112 for oid".to_string()),
        }
    }
}

impl TryFrom<Value> for OidChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// JSON encoding/decoding of `TST-info-based-on-CBOR-time-tag`,
/// see [Epoch Markers Section 4.1.3].
///
/// Use [`TstInfoCborTimeTagCbor`] for CBOR-encoded tokens.
///
/// ```text
/// TST-info-based-on-CBOR-time-tag = {
///   &(version : 0) => v1
///   &(policy : 1) => oid
///   &(messageImprint : 2) => MessageImprint
///   &(serialNumber : 3) => integer
///   &(eTime : 4) => profiled-etime
///   ? &(ordering : 5) => bool .default false
///   ? &(nonce : 6) => integer
///   ? &(tsa : 7) => GeneralName
///   * $$TSTInfoExtensions
/// }
///
/// v1 = 1
/// oid = #6.111(bstr) / #6.112(bstr)
/// ```
///
/// [Epoch Markers Section 4.1.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03#section-4.1.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TstInfoCborTimeTag {
    #[cbor(tag = "0", value = "Integer")]
    pub version: u8,
    #[cbor(tag = "1")]
    pub policy: OidChoice,
    #[cbor(tag = "2", cbor = "true")]
    pub message_imprint: MessageImprint,
    #[cbor(tag = "3", value = "Integer")]
    pub serial_number: i64,
    #[cbor(tag = "4", cbor = "true")]
    pub etime: ProfiledEtime,
    #[cbor(tag = "5", value = "Bool")]
    pub ordering: Option<bool>,
    #[cbor(tag = "6", value = "Integer")]
    pub nonce: Option<i64>,
    #[cbor(tag = "7", cbor = "true")]
    pub tsa: Option<GeneralName>,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}
