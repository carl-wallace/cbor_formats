//! Choice-based structs from the Concise Trust Anchor Store (CoTS) spec

use ciborium::value::Value;
use serde::{Deserialize, Serialize};

use alloc::string::String;

use num_enum::TryFromPrimitive;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
// $concise-tag-type-choice /= #6.999(bytes .cbor concise-ta-stores)

// $pkix-ta-type /= tastore.pkix-cert-type
// $pkix-ta-type /= tastore.pkix-tainfo-type
// $pkix-ta-type /= tastore.pkix-spki-type
// tastore.pkix-cert-type = 0
// tastore.pkix-tainfo-type = 1
// tastore.pkix-spki-type = 2
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PkixTaType {
    Known(PkixTaTypeKnown),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr, TryFromPrimitive)]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i64)]
pub enum PkixTaTypeKnown {
    Cert = 0,
    TaInfo = 1,
    Spki = 2,
}
impl TryFrom<Value> for PkixTaType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(i) {
                Ok(vs) => match PkixTaTypeKnown::try_from(vs) {
                    Ok(val) => Ok(PkixTaType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for PkixTaType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => match <ciborium::value::Integer as TryInto<i64>>::try_into(*i) {
                Ok(vs) => match PkixTaTypeKnown::try_from(vs) {
                    Ok(val) => Ok(PkixTaType::Known(val)),
                    Err(_) => Err("".to_string()),
                },
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

// $tas-list-purpose /= "cots"
// $tas-list-purpose /= "corim"
// $tas-list-purpose /= "comid"
// $tas-list-purpose /= "coswid"
// $tas-list-purpose /= "eat"
// $tas-list-purpose /= "key-attestation"
// $tas-list-purpose /= "certificate"
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum TasListPurpose {
    cots,
    corim,
    comid,
    coswid,
    eat,
    key_attestation,
    certificate,
}
impl TryFrom<Value> for TasListPurpose {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => match TasListPurpose::try_from(s) {
                Ok(val) => Ok(val),
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&Value> for TasListPurpose {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => match TasListPurpose::try_from(s) {
                Ok(val) => Ok(val),
                Err(_) => Err("".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<String> for TasListPurpose {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cots" => Ok(TasListPurpose::cots),
            "corim" => Ok(TasListPurpose::corim),
            "comid" => Ok(TasListPurpose::comid),
            "coswid" => Ok(TasListPurpose::coswid),
            "eat" => Ok(TasListPurpose::eat),
            "key-attestation" => Ok(TasListPurpose::key_attestation),
            "certificate" => Ok(TasListPurpose::certificate),
            _ => Err("".to_string()),
        }
    }
}
impl TryFrom<&String> for TasListPurpose {
    type Error = String;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cots" => Ok(TasListPurpose::cots),
            "corim" => Ok(TasListPurpose::corim),
            "comid" => Ok(TasListPurpose::comid),
            "coswid" => Ok(TasListPurpose::coswid),
            "eat" => Ok(TasListPurpose::eat),
            "key-attestation" => Ok(TasListPurpose::key_attestation),
            "certificate" => Ok(TasListPurpose::certificate),
            _ => Err("".to_string()),
        }
    }
}
