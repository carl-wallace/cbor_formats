//! Choice types from the Epoch Markers spec ([draft-ietf-rats-epoch-markers-03]).
//!
//! | CDDL | Rust |
//! |------|------|
//! | `epoch-marker` / `$tagged-epoch-id` | [`EpochMarker`] |
//! | `cbor-time` | [`CborTime`] |
//! | `epoch-tick` | [`EpochTick`] |
//!
//! [draft-ietf-rats-epoch-markers-03]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use ciborium::tag::Required;
use ciborium::value::Value;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::arrays::EpochTickList;
use crate::maps::{ProfiledEtimeCbor, TstInfoCborTimeTagCbor};

/// CBOR time value: `cbor-time = tdate / time / etime`.
///
/// ```text
/// tdate = #6.0(tstr)                   ; RFC 3339 date string
/// time  = #6.1(int / float)            ; POSIX seconds
/// etime = #6.1001({* (int/tstr) => any}) ; extended time (RFC 9581)
/// ```
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum CborTime {
    /// `tdate = #6.0(tstr)` — tagged date string.
    Tdate(Required<String, 0>),
    /// `time = #6.1(int)` — tagged POSIX integer seconds.
    TimeInt(Required<i64, 1>),
    /// `time = #6.1(float)` — tagged POSIX float seconds.
    TimeFloat(Required<f64, 1>),
    /// `etime = #6.1001(...)` — extended time map.
    Etime(Box<ProfiledEtimeCbor>),
}

impl Serialize for CborTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tdate(v) => v.serialize(serializer),
            Self::TimeInt(v) => v.serialize(serializer),
            Self::TimeFloat(v) => v.serialize(serializer),
            Self::Etime(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CborTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        Self::try_from(&value).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<&Value> for CborTime {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(0, inner) => match inner.as_text() {
                Some(s) => Ok(Self::Tdate(Required(s.to_string()))),
                None => Err("expected text inside tag 0 for tdate".to_string()),
            },
            Value::Tag(1, inner) => match inner.as_ref() {
                Value::Integer(i) => {
                    let val: i64 = (*i)
                        .try_into()
                        .map_err(|_| "integer out of range for time".to_string())?;
                    Ok(Self::TimeInt(Required(val)))
                }
                Value::Float(f) => Ok(Self::TimeFloat(Required(*f))),
                _ => Err("expected integer or float inside tag 1 for time".to_string()),
            },
            Value::Tag(1001, _) => {
                let etime = ProfiledEtimeCbor::try_from(value)?;
                Ok(Self::Etime(Box::new(etime)))
            }
            _ => Err("expected tag 0, 1, or 1001 for cbor-time".to_string()),
        }
    }
}

/// Epoch tick: `epoch-tick = tstr / bstr / int`.
///
/// A single opaque nonce-like value shared among epoch consumers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EpochTick {
    Text(String),
    #[serde(with = "serde_bytes")]
    Bytes(Vec<u8>),
    Int(i64),
}

impl TryFrom<&Value> for EpochTick {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Bytes(b) => Ok(Self::Bytes(b.clone())),
            Value::Integer(i) => {
                let val: i64 = (*i)
                    .try_into()
                    .map_err(|_| "integer out of range for epoch-tick".to_string())?;
                Ok(Self::Int(val))
            }
            _ => Err("expected text, bytes, or integer for epoch-tick".to_string()),
        }
    }
}

impl TryFrom<Value> for EpochTick {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// Top-level epoch marker: `epoch-marker = $tagged-epoch-id`.
///
/// ```text
/// $tagged-epoch-id /= cbor-time
/// $tagged-epoch-id /= #6.26980(classical-rfc3161-TST-info)
/// $tagged-epoch-id /= #6.26981(TST-info-based-on-CBOR-time-tag)
/// $tagged-epoch-id /= #6.26982(epoch-tick)
/// $tagged-epoch-id /= #6.26983(epoch-tick-list)
/// $tagged-epoch-id /= #6.26984(strictly-monotonic-counter)
/// ```
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum EpochMarker {
    /// `cbor-time` — tdate (#6.0), time (#6.1), or etime (#6.1001).
    CborTime(CborTime),
    /// `#6.26980(classical-rfc3161-TST-info)` — DER-encoded TSTInfo bytes.
    ClassicalTstInfo(Required<serde_bytes::ByteBuf, 26980>),
    /// `#6.26981(TST-info-based-on-CBOR-time-tag)` — CBOR-encoded TSTInfo.
    CborTstInfo(Box<Required<TstInfoCborTimeTagCbor, 26981>>),
    /// `#6.26982(epoch-tick)` — single opaque tick value.
    EpochTick(Required<EpochTick, 26982>),
    /// `#6.26983(epoch-tick-list)` — list of tick values.
    EpochTickList(Required<EpochTickList, 26983>),
    /// `#6.26984(strictly-monotonic-counter)` — monotonic counter.
    StrictlyMonotonicCounter(Required<u64, 26984>),
}

impl Serialize for EpochMarker {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CborTime(v) => v.serialize(serializer),
            Self::ClassicalTstInfo(v) => v.serialize(serializer),
            Self::CborTstInfo(v) => v.as_ref().serialize(serializer),
            Self::EpochTick(v) => v.serialize(serializer),
            Self::EpochTickList(v) => v.serialize(serializer),
            Self::StrictlyMonotonicCounter(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for EpochMarker {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        Self::try_from(&value).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<&Value> for EpochMarker {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(0, _) | Value::Tag(1, _) | Value::Tag(1001, _) => {
                let ct = CborTime::try_from(value)?;
                Ok(Self::CborTime(ct))
            }
            Value::Tag(26980, inner) => match inner.as_bytes() {
                Some(b) => Ok(Self::ClassicalTstInfo(Required(
                    serde_bytes::ByteBuf::from(b.clone()),
                ))),
                None => Err("expected bytes inside tag 26980".to_string()),
            },
            Value::Tag(26981, inner) => {
                let tst = TstInfoCborTimeTagCbor::try_from(inner.as_ref())?;
                Ok(Self::CborTstInfo(Box::new(Required(tst))))
            }
            Value::Tag(26982, inner) => {
                let tick = EpochTick::try_from(inner.as_ref())?;
                Ok(Self::EpochTick(Required(tick)))
            }
            Value::Tag(26983, inner) => match inner.as_array() {
                Some(arr) => {
                    let items: Result<Vec<EpochTick>, String> =
                        arr.iter().map(EpochTick::try_from).collect();
                    Ok(Self::EpochTickList(Required(EpochTickList(items?))))
                }
                None => Err("expected array inside tag 26983".to_string()),
            },
            Value::Tag(26984, inner) => match inner.as_integer() {
                Some(i) => {
                    let val: u64 = i
                        .try_into()
                        .map_err(|_| "expected uint for strictly-monotonic-counter".to_string())?;
                    Ok(Self::StrictlyMonotonicCounter(Required(val)))
                }
                None => Err("expected uint inside tag 26984".to_string()),
            },
            _ => Err(format!("unrecognized epoch-marker value: {:?}", value)),
        }
    }
}

impl TryFrom<Value> for EpochMarker {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
