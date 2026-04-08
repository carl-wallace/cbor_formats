//! Choice-based types from the CMW specification (draft-ietf-rats-msg-wrap-23)

use ciborium::value::Value;
use serde::{Deserialize, Serialize};

use alloc::string::{String, ToString};

use num_enum::TryFromPrimitive;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

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
