//! Choice-based structs from the Entity Attestation Token (EAT) spec

use ciborium::value::Value;
use serde::{Deserialize, Serialize};

use alloc::string::{String, ToString};
use num_enum::TryFromPrimitive;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

// BUNDLE-Messages = BUNDLE-Tagged-Message / BUNDLE-Untagged-Message

// CBOR-Nested-Token =
//     JSON-Token-Inside-CBOR-Token /
//     CBOR-Token-Inside-CBOR-Token

// CBOR-Submodule = Claims-Set / CBOR-Nested-Token /
//             Detached-Submodule-Digest

// ; The CBOR tag mechanism is used to select between the various types
// ; of CBOR encoded tokens.
// CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token

// Claim-Label = int / text
// Use TextOrInt type from common

// CWT-Messages = CWT-Tagged-Message / CWT-Untagged-Message
// CWT-Untagged-Message = COSE_Messages

// debug-status-type = ds-enabled /
//                     disabled /
//                     disabled-since-boot /
//                     disabled-permanently /
//                     disabled-fully-and-permanently
//
// ds-enabled                     = 0
// disabled                       = 1
// disabled-since-boot            = 2
// disabled-permanently           = 3
// disabled-fully-and-permanently = 4
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum DebugStatusType {
    Known(DebugStatusTypeKwown),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum DebugStatusTypeKwown {
    Enabled = 0,
    Disabled = 1,
    DisabledSinceBoot = 2,
    DisabledPermanently = 3,
    DisabledFullyAndPermanently = 4,
}

impl TryFrom<Value> for DebugStatusType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match DebugStatusTypeKwown::try_from(vs) {
                    Ok(val) => Ok(DebugStatusType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for DebugStatusType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match DebugStatusTypeKwown::try_from(vs) {
                    Ok(val) => Ok(DebugStatusType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// EAT-CBOR-Token = $EAT-CBOR-Tagged-Token / $EAT-CBOR-Untagged-Token
//
// $EAT-CBOR-Tagged-Token /= CWT-Tagged-Message
// $EAT-CBOR-Tagged-Token /= BUNDLE-Tagged-Message
//
// $EAT-CBOR-Untagged-Token /= CWT-Untagged-Message
// $EAT-CBOR-Untagged-Token /= BUNDLE-Untagged-Message
//
// EAT-JSON-Token = $EAT-JSON-Token-Formats
//
// $EAT-JSON-Token-Formats /= JWT-Message
// $EAT-JSON-Token-Formats /= BUNDLE-Untagged-Message

// intended-use-type = generic /
//                     registration /
//                     provisioning /
//                     csr /
//                     pop
//
// generic      = 1
// registration = 2
// provisioning = 3
// csr          = 4
// pop          = 5
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum IntendedUseType {
    Known(IntendedUseTypeKnown),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum IntendedUseTypeKnown {
    Generic = 1,
    Registration = 2,
    Provisioning = 3,
    Csr = 4,
    Pop = 5,
}

impl TryFrom<Value> for IntendedUseType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match IntendedUseTypeKnown::try_from(vs) {
                    Ok(val) => Ok(IntendedUseType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for IntendedUseType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match IntendedUseTypeKnown::try_from(vs) {
                    Ok(val) => Ok(IntendedUseType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// $JSON-Selector-Type /= "JWT" / "CBOR" / "BUNDLE" / "DIGEST"
// $JSON-Selector-Value /= JWT-Message /
//                   CBOR-Token-Inside-JSON-Token /
//                   Detached-EAT-Bundle /
//                   Detached-Submodule-Digest

// JSON-Submodule = Claims-Set / JSON-Selector

// ; The contents of this text string MUST be a JSON-encoded
// ; JSON-Selector.  See the definition of JSON-Selector. The
// ; Detached-Submodule-Digest option MUST NOT be used when included
// ; in a CBOR token
// JSON-Token-Inside-CBOR-Token = tstr

// $manifest-body-cbor /= cyclone-dx-json
// $manifest-body-cbor /= cyclone-dx-xml
// $manifest-body-json /= cyclone-dx-json
// $manifest-body-json /= cyclone-dx-xml
// $manifest-body-cbor /= spdx-json
// $manifest-body-json /= spdx-json
// $manifest-body-cbor /= bytes .cbor SUIT_Envelope
// $manifest-body-json /= base64-url-text
// $manifest-body-cbor /= bytes .cbor untagged-coswid
// $manifest-body-json /= base64-url-text

// $measurements-body-cbor /= bytes .cbor untagged-coswid
// $measurements-body-json /= base64-url-text

// result-type = comparison-successful /
//               comparison-fail /
//               comparison-not-run /
//               measurement-absent
//
// comparison-successful    = 1
// comparison-fail          = 2
// comparison-not-run       = 3
// measurement-absent       = 4
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ResultType {
    Known(ResultTypeKnown),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum ResultTypeKnown {
    Successful = 1,
    Fail = 2,
    NotRun = 3,
    Absent = 4,
}

impl TryFrom<Value> for ResultType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match ResultTypeKnown::try_from(vs) {
                    Ok(val) => Ok(ResultType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for ResultType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match ResultTypeKnown::try_from(vs) {
                    Ok(val) => Ok(ResultType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
