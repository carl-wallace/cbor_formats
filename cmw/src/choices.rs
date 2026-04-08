//! Choice-based types from the CMW specification (draft-ietf-rats-msg-wrap-23)

use ciborium::tag::Required;
use ciborium::value::Value;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize, Serializer};

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;

use common::BytesType;
use cose::arrays::CoseSign1Cbor;
use num_enum::TryFromPrimitive;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

use crate::arrays::{CborRecordCbor, JsonRecordCbor};
use crate::maps::{CborCollection, JsonCollection};

// cmw-indicator = uint .bits cm-type
// cm-type = &(
//     reference-values: 0
//     endorsements: 1
//     evidence: 2
//     attestation-results: 3
//     appraisal-policy: 4
// )

/// Known conceptual message type indicator bit positions, as defined by the `cm-type` group
/// in [CMW Section 3.1.1].
///
/// These are bit positions within the `cmw-indicator` bitmask.
///
/// [CMW Section 3.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3.1.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum CmType {
    ReferenceValues = 0,
    Endorsements = 1,
    Evidence = 2,
    AttestationResults = 3,
    AppraisalPolicy = 4,
}

// coap-content-format-or-media-type = coap-content-format / media-type
// coap-content-format = uint
// media-type = tstr

/// A choice between a CoAP Content-Format number and a media type string, as defined by
/// `coap-content-format-or-media-type` in [CMW Section 3.1].
///
/// [CMW Section 3.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CoapContentFormatOrMediaType {
    ContentFormat(u16),
    MediaType(String),
}

impl TryFrom<&Value> for CoapContentFormatOrMediaType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::MediaType(k.clone())),
            Value::Integer(k) => {
                let val: u16 = (*k).try_into().map_err(|_| {
                    "Integer value out of range for CoapContentFormatOrMediaType".to_string()
                })?;
                Ok(Self::ContentFormat(val))
            }
            _ => Err("Failed to parse value as a CoapContentFormatOrMediaType".to_string()),
        }
    }
}

impl TryFrom<Value> for CoapContentFormatOrMediaType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::MediaType(k)),
            Value::Integer(k) => {
                let val: u16 = k.try_into().map_err(|_| {
                    "Integer value out of range for CoapContentFormatOrMediaType".to_string()
                })?;
                Ok(Self::ContentFormat(val))
            }
            _ => Err("Failed to parse value as a CoapContentFormatOrMediaType".to_string()),
        }
    }
}

// cmw-indicator = uint .bits cm-type

/// Wrapper for the CMW indicator bitmask, as defined by `cmw-indicator` in [CMW Section 3.1.1].
///
/// The indicator is a `uint` whose bits encode one or more `cm-type` values.
///
/// [CMW Section 3.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3.1.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CmwIndicator(pub u64);

impl CmwIndicator {
    /// Creates a new empty indicator with no bits set.
    pub fn new() -> Self {
        Self(0)
    }

    /// Creates an indicator from a single `CmType`.
    pub fn from_type(cm_type: CmType) -> Self {
        Self(1 << cm_type as u64)
    }

    /// Creates an indicator from multiple `CmType` values.
    pub fn from_types(types: &[CmType]) -> Self {
        let mut bits = 0u64;
        for t in types {
            bits |= 1 << t.clone() as u64;
        }
        Self(bits)
    }

    /// Returns `true` if the given `CmType` bit is set.
    pub fn has(&self, cm_type: CmType) -> bool {
        self.0 & (1 << cm_type as u64) != 0
    }

    /// Sets the bit for the given `CmType`.
    pub fn set(&mut self, cm_type: CmType) {
        self.0 |= 1 << cm_type as u64;
    }

    /// Clears the bit for the given `CmType`.
    pub fn clear(&mut self, cm_type: CmType) {
        self.0 &= !(1 << cm_type as u64);
    }

    /// Returns an iterator over the `CmType` values that are set.
    pub fn iter(&self) -> impl Iterator<Item = CmType> + '_ {
        [
            CmType::ReferenceValues,
            CmType::Endorsements,
            CmType::Evidence,
            CmType::AttestationResults,
            CmType::AppraisalPolicy,
        ]
        .into_iter()
        .filter(|t| self.has(t.clone()))
    }
}

impl Default for CmwIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Value> for CmwIndicator {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(k) => {
                let val: u64 = (*k)
                    .try_into()
                    .map_err(|_| "Integer value out of range for CmwIndicator".to_string())?;
                Ok(Self(val))
            }
            _ => Err("Failed to parse value as a CmwIndicator".to_string()),
        }
    }
}

impl TryFrom<Value> for CmwIndicator {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(k) => {
                let val: u64 = k
                    .try_into()
                    .map_err(|_| "Integer value out of range for CmwIndicator".to_string())?;
                Ok(Self(val))
            }
            _ => Err("Failed to parse value as a CmwIndicator".to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Top-level CMW types (Section 3)
// ---------------------------------------------------------------------------

/// Key type for entries in a `cbor-collection` map.
///
/// CBOR collection labels can be integers or text strings.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CborCollectionKey {
    /// Integer label
    Int(i64),
    /// Text label
    Text(String),
}

/// The `cbor-cmw` choice type from [CMW Section 3].
///
/// ```text
/// cbor-cmw = cbor-record / cbor-collection / $cbor-tag
/// ```
///
/// [CMW Section 3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3
#[derive(Clone, Debug, PartialEq)]
pub enum CborCmw {
    /// `#6.1668547091(cbor-collection)` — tagged collection
    TagCollection(Required<Box<CborCollection>, 1668547091>),
    /// `#6.1668547092(signed-cbor-cmw)` — tagged COSE_Sign1
    TagSigned(Required<CoseSign1Cbor, 1668547092>),
    /// `#6.1668547093(bstr)` — tagged CMW+JSON collection data
    TagCmwJsonCollectionData(Required<BytesType, 1668547093>),
    /// `#6.1668547094(bstr)` — tagged CMW+JWS data
    TagCmwJwsData(Required<BytesType, 1668547094>),
    /// Untagged `cbor-record`
    Record(CborRecordCbor),
    /// Untagged `cbor-collection`
    Collection(CborCollection),
}

/// Tag numbers for CMW CBOR tags
const TAG_COLLECTION: u64 = 1668547091;
const TAG_SIGNED: u64 = 1668547092;
const TAG_CMW_JSON_COLLECTION_DATA: u64 = 1668547093;
const TAG_CMW_JWS_DATA: u64 = 1668547094;

/// Helper: serialize a ciborium::Value to bytes, then deserialize as T.
fn value_to_type<T: serde::de::DeserializeOwned>(val: &Value) -> Result<T, String> {
    let mut buf = vec![];
    ciborium::ser::into_writer(val, &mut buf).map_err(|e| format!("re-serialize: {e}"))?;
    ciborium::de::from_reader(buf.as_slice()).map_err(|e| format!("re-deserialize: {e}"))
}

impl Serialize for CborCmw {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CborCmw::TagCollection(v) => v.serialize(serializer),
            CborCmw::TagSigned(v) => v.serialize(serializer),
            CborCmw::TagCmwJsonCollectionData(v) => v.serialize(serializer),
            CborCmw::TagCmwJwsData(v) => v.serialize(serializer),
            CborCmw::Record(v) => v.serialize(serializer),
            CborCmw::Collection(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CborCmw {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        match &value {
            Value::Tag(tag, inner) => match *tag {
                TAG_COLLECTION => {
                    let col: CborCollection = value_to_type(inner).map_err(de::Error::custom)?;
                    Ok(CborCmw::TagCollection(Required(Box::new(col))))
                }
                TAG_SIGNED => {
                    let sign1: CoseSign1Cbor = value_to_type(inner).map_err(de::Error::custom)?;
                    Ok(CborCmw::TagSigned(Required(sign1)))
                }
                TAG_CMW_JSON_COLLECTION_DATA => {
                    let bytes: BytesType = value_to_type(inner).map_err(de::Error::custom)?;
                    Ok(CborCmw::TagCmwJsonCollectionData(Required(bytes)))
                }
                TAG_CMW_JWS_DATA => {
                    let bytes: BytesType = value_to_type(inner).map_err(de::Error::custom)?;
                    Ok(CborCmw::TagCmwJwsData(Required(bytes)))
                }
                other => Err(de::Error::custom(format!("unknown cbor-cmw tag: {other}"))),
            },
            Value::Array(_) => {
                let record: CborRecordCbor = value_to_type(&value).map_err(de::Error::custom)?;
                Ok(CborCmw::Record(record))
            }
            Value::Map(_) => {
                let col: CborCollection = value_to_type(&value).map_err(de::Error::custom)?;
                Ok(CborCmw::Collection(col))
            }
            _ => Err(de::Error::custom(
                "expected tag, array, or map for cbor-cmw",
            )),
        }
    }
}

/// The `json-cmw` choice type from [CMW Section 3].
///
/// ```text
/// json-cmw = json-record / json-collection
/// ```
///
/// [CMW Section 3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3
#[derive(Clone, Debug, PartialEq)]
pub enum JsonCmw {
    /// `json-record`
    Record(JsonRecordCbor),
    /// `json-collection`
    Collection(JsonCollection),
}

impl Serialize for JsonCmw {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            JsonCmw::Record(v) => v.serialize(serializer),
            JsonCmw::Collection(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for JsonCmw {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        match &value {
            Value::Array(_) => {
                let record: JsonRecordCbor = value_to_type(&value).map_err(de::Error::custom)?;
                Ok(JsonCmw::Record(record))
            }
            Value::Map(_) => {
                let col: JsonCollection = value_to_type(&value).map_err(de::Error::custom)?;
                Ok(JsonCmw::Collection(col))
            }
            _ => Err(de::Error::custom("expected array or map for json-cmw")),
        }
    }
}

/// The top-level `cmw` choice type from [CMW Section 3].
///
/// ```text
/// cmw = json-cmw / cbor-cmw
/// ```
///
/// [CMW Section 3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3
#[derive(Clone, Debug, PartialEq)]
pub enum Cmw {
    /// CBOR-encoded CMW
    Cbor(CborCmw),
    /// JSON-encoded CMW
    Json(JsonCmw),
}

impl Serialize for Cmw {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Cmw::Cbor(v) => v.serialize(serializer),
            Cmw::Json(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Cmw {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Try as CborCmw first (handles tags, then arrays/maps).
        // CborCmw is a superset of JsonCmw for CBOR encoding purposes,
        // since json-record and cbor-record are both arrays, and
        // json-collection and cbor-collection are both maps.
        let value = Value::deserialize(deserializer)?;
        // Re-serialize the value and try CborCmw
        let mut buf = vec![];
        ciborium::ser::into_writer(&value, &mut buf).map_err(de::Error::custom)?;
        if let Ok(cbor_cmw) = ciborium::de::from_reader::<CborCmw, _>(buf.as_slice()) {
            return Ok(Cmw::Cbor(cbor_cmw));
        }
        if let Ok(json_cmw) = ciborium::de::from_reader::<JsonCmw, _>(buf.as_slice()) {
            return Ok(Cmw::Json(json_cmw));
        }
        Err(de::Error::custom("value does not match any cmw variant"))
    }
}
