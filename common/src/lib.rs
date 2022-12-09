#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
//#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
pub mod arrays;
pub mod choices;
pub mod tuple;
pub mod tuple_map;

pub use tuple::*;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ciborium::tag::Required;
use ciborium::value::{Integer, Value};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum BytesType {
    #[serde(with = "serde_bytes")]
    Bytes(Vec<u8>),
}
impl TryFrom<&Value> for BytesType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Bytes(k.clone())),
            _ => Err("Failed to parse value as a BytesType".to_string()),
        }
    }
}
impl TryFrom<Value> for BytesType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Bytes(k)),
            _ => Err("Failed to parse value as a BytesType".to_string()),
        }
    }
}

/// Uri type
pub type Uri = String;

///tagged-int-type = #6.551(int)
pub type TaggedIntType = Required<IntType, 551>;

//todo the cocli tests use tag 600 here
///tagged-int-type = #6.600(int)
pub type TaggedIntType2 = Required<IntType, 600>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum NonceType {
    One(BytesType),
    More(Vec<BytesType>),
}
impl TryFrom<&Value> for NonceType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::One(BytesType::Bytes(k.clone()))),
            Value::Array(k) => Ok(Self::More(
                k.iter()
                    .map(|m| BytesType::Bytes(m.as_bytes().unwrap().clone()))
                    .collect(),
            )),
            _ => Err("Failed to parse value as a NonceType".to_string()),
        }
    }
}

//todo the corim code emits bytes, but the spec says int
/// type to serve as target for TaggedIntType
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum IntType {
    #[serde(with = "serde_bytes")]
    Int(Vec<u8>),
}
impl TryFrom<&Value> for IntType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Int(k.clone())),
            _ => Err("Failed to parse value as an IntType".to_string()),
        }
    }
}

/// tagged-pkix-base64-key-type = #6.554(tstr)
pub type TaggedPkixBase64KeyType = Required<String, 554>;

/// tagged-pkix-base64-cert-type = #6.555(tstr)
pub type TaggedPkixBase64CertType = Required<String, 555>;

/// tagged-pkix-base64-cert-path-type = #6.556(tstr)
pub type TaggedPkixBase64CertPathType = Required<String, 556>;

/// ueid-type = bytes .size 33
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum UeidType {
    #[serde(with = "serde_bytes")]
    Ueid(Vec<u8>),
}
impl TryFrom<&Value> for UeidType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Ueid(k.clone())),
            _ => Err("Failed to parse value as a UeidType".to_string()),
        }
    }
}

/// tagged-ueid-type = #6.550(ueid-type)
pub type TaggedUeidType = Required<UeidType, 550>;

/// oid-type = bytes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OidType {
    #[serde(with = "serde_bytes")]
    Oid(Vec<u8>),
}
/// tagged-oid-type = #6.111(oid-type)
pub type TaggedOidTypeCbor = Required<OidType, 111>;

#[allow(missing_docs)]
pub type TaggedOidType = OidType;

//todo size limit
/// uuid-type = bytes .size 16
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum UuidType {
    #[serde(with = "serde_bytes")]
    Uuid(Vec<u8>),
}
impl TryFrom<&Value> for UuidType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Uuid(k.clone())),
            _ => Err("Failed to parse value as a UuidType".to_string()),
        }
    }
}

/// tagged-uuid-type = #6.37(uuid-type)
pub type TaggedUuidType = Required<UuidType, 37>;

//pub type TaggedUriType = Required<Uri, 32>;
#[allow(missing_docs)]
pub type TaggedUriType = Uri;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TaggedUriTypeCbor {
    U(Required<Uri, 32>),
}
impl TryFrom<&Value> for TaggedUriTypeCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(32, k) => Ok(Self::U(Required(k.as_text().unwrap().to_string()))),
            _ => Err("Failed to parse value as a TaggedUriTypeCbor".to_string()),
        }
    }
}
impl TryFrom<&String> for TaggedUriTypeCbor {
    type Error = String;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Self::U(Required(value.clone())))
    }
}
impl TryFrom<String> for TaggedUriTypeCbor {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::U(Required(value)))
    }
}
impl TryFrom<TaggedUriTypeCbor> for String {
    type Error = String;
    fn try_from(value: TaggedUriTypeCbor) -> Result<Self, Self::Error> {
        match value {
            TaggedUriTypeCbor::U(u) => Ok(u.0),
        }
    }
}
impl TryFrom<&TaggedUriTypeCbor> for String {
    type Error = String;
    fn try_from(value: &TaggedUriTypeCbor) -> Result<Self, Self::Error> {
        match value {
            TaggedUriTypeCbor::U(u) => Ok(u.0.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OidOrUri {
    U(TaggedUriType),
    O(TaggedOidType),
}
impl TryFrom<&Value> for OidOrUri {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(32, k) => Ok(Self::U(k.as_text().unwrap().to_string())),
            Value::Tag(111, k) => Ok(Self::O(OidType::Oid(k.as_bytes().unwrap().clone()))),
            _ => Err("Failed to parse value as a OidOrUri".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OidOrUriCbor {
    U(TaggedUriTypeCbor),
    O(TaggedOidTypeCbor),
}
impl TryFrom<&Value> for OidOrUriCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(32, k) => Ok(Self::U(TaggedUriTypeCbor::U(Required(
                k.as_text().unwrap().to_string(),
            )))),
            Value::Tag(111, k) => Ok(Self::O(TaggedOidTypeCbor {
                0: OidType::Oid(k.as_bytes().unwrap().clone()),
            })),
            _ => Err("Failed to parse value as a OidOrUriCbor".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PkixBase64Type {
    Base64(String),
}
impl TryFrom<&Value> for PkixBase64Type {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Base64(k.to_string())),
            _ => Err("Failed to parse value as a PkixBase64Type".to_string()),
        }
    }
}
impl TryFrom<Value> for PkixBase64Type {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Base64(k)),
            _ => Err("Failed to parse value as a PkixBase64Type".to_string()),
        }
    }
}

// ; ==== common-types.cddl ====
// ; The same as the standard time, but floating point
// ; is not allowed.
// time-int = #6.1(int)
//pub type time = Required<i64, 1>;
#[allow(missing_docs)]
pub type Time = i64;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TimeCbor {
    T(Required<i64, 1>),
}
impl TryFrom<&Value> for TimeCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(_i, k) => Ok(Self::T(Required(
                k.as_integer().unwrap().try_into().unwrap(),
            ))),
            _ => Err("Failed to parse value as a TimeCbor".to_string()),
        }
    }
}
impl TryFrom<&TimeCbor> for i64 {
    type Error = String;
    fn try_from(value: &TimeCbor) -> Result<Self, Self::Error> {
        match value {
            TimeCbor::T(k) => Ok(k.0),
        }
    }
}
impl TryFrom<i64> for TimeCbor {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self::T(Required(value)))
    }
}
impl TryFrom<&i64> for TimeCbor {
    type Error = String;
    fn try_from(value: &i64) -> Result<Self, Self::Error> {
        Ok(Self::T(Required(*value)))
    }
}

// ; binary data that works for both JSON and CBOR.
// binary-data = bstr
//
// base64-url-text = tstr ; .regexp "[A-Za-z0-9_=-]+"
//
//
// ; OID for both JSON and CBOR
// general-oid = ~oid
//
// ; This is a normative definition for the encoding of an OID
// ; as a text string in JSON as used by EAT
// json-oid = tstr ; .regexp "([0-2])((\.0)|(\.[1-9][0-9]*))*"
//
//
// ; URI for both JSON and CBOR
// general-uri = ~uri
//
//
// ; CoAP Content-Format from RFC 7252 section 12.3
// coap-content-format = uint .le 65535

/// svn-type = uint
/// svn = svn-type
/// min-svn = svn-type
/// tagged-svn = #6.552(svn)
pub type TaggedSvn = Required<u64, 552>;

/// tagged-min-svn = #6.553(min-svn)
pub type TaggedMinSvn = Required<u64, 553>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TextOrBinary {
    #[serde(with = "serde_bytes")]
    Binary(Vec<u8>),
    Text(String),
}
impl TryFrom<&Value> for TextOrBinary {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Text(k.clone())),
            Value::Bytes(k) => Ok(Self::Binary(k.clone())),
            _ => Err("Failed to parse value as a TextOrBinary".to_string()),
        }
    }
}
impl TryFrom<Value> for TextOrBinary {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Text(k)),
            Value::Bytes(k) => Ok(Self::Binary(k)),
            _ => Err("Failed to parse value as a TextOrBinary".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PkixCa {
    #[serde(with = "serde_bytes")]
    Binary(Vec<u8>),
}
impl TryFrom<&Value> for PkixCa {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Binary(k.to_vec())),
            _ => Err("Failed to parse value as a PkixCa".to_string()),
        }
    }
}
impl TryFrom<Value> for PkixCa {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => Ok(Self::Binary(k)),
            _ => Err("Failed to parse value as a PkixCa".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TextOrInt {
    Int(i64),
    Text(String),
}
impl TryFrom<&Value> for TextOrInt {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Text(k.clone())),
            Value::Integer(k) => Ok(Self::Int(Integer::try_into(*k).unwrap())),
            _ => Err("Failed to parse value as a TextOrInt".to_string()),
        }
    }
}
impl TryFrom<Value> for TextOrInt {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Text(k)),
            Value::Integer(k) => Ok(Self::Int(Integer::try_into(k).unwrap())),
            _ => Err("Failed to parse value as a TextOrInt".to_string()),
        }
    }
}
