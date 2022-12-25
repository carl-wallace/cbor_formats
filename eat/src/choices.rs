//! Choice-based structs

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use ciborium::value::Value;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// BUNDLE-Tagged-Message   = #6.602(BUNDLE-Untagged-Message)
// BUNDLE-Untagged-Message = Detached-EAT-Bundle
// BUNDLE-Messages = BUNDLE-Tagged-Message / BUNDLE-Untagged-Message

// Claim-Label = int / text
// Use TextOrInt type from common

// CWT-Messages = CWT-Tagged-Message / CWT-Untagged-Message
// CWT-Untagged-Message = COSE_Messages

/// CBOR and JSON encoding/decoding of `debug-status-type`, see [EAT Section 4.2.9].
///
/// ```text
/// debug-status-type = ds-enabled /
///                     disabled /
///                     disabled-since-boot /
///                     disabled-permanently /
///                     disabled-fully-and-permanently
///
/// ds-enabled                     = 0
/// disabled                       = 1
/// disabled-since-boot            = 2
/// disabled-permanently           = 3
/// disabled-fully-and-permanently = 4
/// ```
/// [EAT Section 4.2.9]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.2.9
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i8)]
pub enum DebugStatusType {
    Enabled = 0,
    Disabled = 1,
    DisabledSinceBoot = 2,
    DisabledPermanently = 3,
    DisabledFullyAndPermanently = 4,
}
impl TryFrom<Value> for DebugStatusType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        DebugStatusType::try_from(&value)
    }
}
impl TryFrom<&Value> for DebugStatusType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i8>>::try_into(*i) {
                Ok(vs) => match DebugStatusType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

/// CBOR and JSON encoding/decoding of `intended-use-type`, see [EAT Section 4.3.3].
///
/// ```text
/// intended-use-type = generic /
///                     registration /
///                     provisioning /
///                     csr /
///                     pop
///
/// generic      = 1
/// registration = 2
/// provisioning = 3
/// csr          = 4
/// pop          = 5
/// ```
/// [EAT Section 4.3.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.3.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i8)]
pub enum IntendedUseType {
    Generic = 1,
    Registration = 2,
    Provisioning = 3,
    Csr = 4,
    Pop = 5,
}

impl TryFrom<Value> for IntendedUseType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        IntendedUseType::try_from(&value)
    }
}
impl TryFrom<&Value> for IntendedUseType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i8>>::try_into(*i) {
                Ok(vs) => match IntendedUseType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// The EAT specification provides CBOR-specific and JSON-specific definitions for the type
// referenced by the manifest-format::content-format definition. The definitions are as follows:
//
//     spdx-json = text
//     cyclone-dx-json = text
//     cyclone-dx-xml  = text
//
//     $manifest-body-cbor /= cyclone-dx-json
//     $manifest-body-cbor /= cyclone-dx-xml
//     $manifest-body-cbor /= spdx-json
//     $manifest-body-cbor /= bytes .cbor SUIT_Envelope
//     $manifest-body-cbor /= bytes .cbor untagged-coswid
//
//     $manifest-body-json /= cyclone-dx-json
//     $manifest-body-json /= cyclone-dx-xml
//     $manifest-body-json /= spdx-json
//     $manifest-body-json /= base64-url-text
//     $manifest-body-json /= base64-url-text
//
// Setting aside the .cbor control operator, manifest-body-cbor is this:
//     $manifest-body-cbor /= text / text / text / bytes / bytes
//
// Setting aside type aliases, manifest-body-json is this:
//     $manifest-body-json /= text / text / text / base64-url-text / base64-url-text
//
// Since these definitions essentially boil down to the same things, are extensible and have the
// type indicated in the  manifest-format::content-type field, the  TextOrBinary construction is
// used instead of types for $manifest-body-cbor and $manifest-body-json.

// $measurements-body-cbor /= bytes .cbor untagged-coswid
// $measurements-body-json /= base64-url-text
// Punting on the socket for now and just using TextOrBinary in measurements-format

/// CBOR and JSON encoding/decoding of types used by the `oemid` claim, see [EAT Section 4.2.3].
///
/// ```text
/// oemid-label => oemid-pen / oemid-ieee / oemid-random
/// ```
/// [EAT Section 4.2.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.2.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Oemid {
    Pen(i64),
    #[serde(with = "serde_bytes")]
    Ieee(Vec<u8>),
    #[serde(with = "serde_bytes")]
    Random(Vec<u8>),
}

impl TryFrom<Value> for Oemid {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(val) => Ok(Oemid::Pen(val)),
                Err(_) => Err("".to_string()),
            },
            Value::Bytes(v) => {
                if 3 == v.len() {
                    Ok(Oemid::Ieee(v))
                } else if 16 == v.len() {
                    Ok(Oemid::Random(v))
                } else {
                    Err("".to_string())
                }
            }
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for Oemid {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(val) => Ok(Oemid::Pen(val)),
                Err(_) => Err("".to_string()),
            },
            Value::Bytes(v) => {
                if 3 == v.len() {
                    Ok(Oemid::Ieee(v.clone()))
                } else if 16 == v.len() {
                    Ok(Oemid::Random(v.clone()))
                } else {
                    Err("".to_string())
                }
            }
            _ => Err("".to_string()),
        }
    }
}

/// CBOR and JSON encoding/decoding of `result-type`, see [EAT Section 4.2.17].
///
/// ```text
/// result-type = comparison-successful /
///               comparison-fail /
///               comparison-not-run /
///               measurement-absent
///
/// comparison-successful    = 1
/// comparison-fail          = 2
/// comparison-not-run       = 3
/// measurement-absent       = 4
/// ```
/// [EAT Section 4.2.17]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.2.17
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i8)]
pub enum ResultType {
    Successful = 1,
    Fail = 2,
    NotRun = 3,
    Absent = 4,
}

impl TryFrom<Value> for ResultType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        ResultType::try_from(&value)
    }
}
impl TryFrom<&Value> for ResultType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i8>>::try_into(*i) {
                Ok(vs) => match ResultType::try_from(vs) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
