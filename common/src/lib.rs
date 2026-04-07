#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
pub mod arrays;
pub mod choices;
pub mod tuple;
pub mod tuple_map;

pub use tuple::*;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ciborium::tag::Required;
use ciborium::value::{Integer, Value};
use serde::{Deserialize, Serialize};

/// Wrapper enum for a CBOR byte string value.
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

/// The `tagged-bytes` type is defined in [CoRIM Section 7.8].
///
/// ```text
/// tagged-bytes = #6.560(bytes)
/// ```
///
/// [CoRIM Section 7.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.8
pub type TaggedBytes = Required<BytesType, 560>;

/// The `tagged-key-thumbprint-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-key-thumbprint-type = #6.557(hash-entry)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedKeyThumbprintType = Required<arrays::HashEntry, 557>;

/// The `tagged-cose-key-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-cose-key-type = #6.558(bytes)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedCoseKeyType = Required<BytesType, 558>;

/// The `tagged-cert-thumbprint-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-cert-thumbprint-type = #6.559(hash-entry)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedCertThumbprintType = Required<arrays::HashEntry, 559>;

/// The `tagged-cert-path-thumbprint-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-cert-path-thumbprint-type = #6.561(hash-entry)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedCertPathThumbprintType = Required<arrays::HashEntry, 561>;

/// The `tagged-pkix-asn1-der-cert-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-pkix-asn1-der-cert-type = #6.562(bytes)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedPkixAsn1DerCertType = Required<BytesType, 562>;

/// The `tagged-masked-raw-value` is defined in [CoRIM Section 5.1.4.5.6].
///
/// ```text
/// tagged-masked-raw-value = #6.563(masked-raw-value)
/// ```
///
/// [CoRIM Section 5.1.4.5.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.6
pub type TaggedMaskedRawValue = Required<arrays::MaskedRawValueCbor, 563>;

/// The `tagged-int-range` is defined in [CoRIM Section 5.1.4.8].
///
/// ```text
/// tagged-int-range = #6.564(int-range)
/// ```
///
/// [CoRIM Section 5.1.4.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.8
pub type TaggedIntRange = Required<arrays::IntRangeCbor, 564>;

/// The `digests-type` is defined in [CoRIM Section 7.7].
///
/// ```text
/// digests-type = [ + hash-entry ]
/// ```
///
/// [CoRIM Section 7.7]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.7
pub type DigestsType = Vec<arrays::HashEntry>;

/// The `integrity-registers` type is defined in [CoRIM Section 5.1.4.7].
///
/// ```text
/// integrity-registers = { + $measured-element-type-choice => digests-type }
/// ```
///
/// Represented as a Vec of (Value, Value) pairs since keys can be uint or text.
///
/// [CoRIM Section 5.1.4.7]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.7
pub type IntegrityRegisters = Vec<(TextOrInt, DigestsType)>;

/// Represents a nonce as either a single byte string or an array of byte strings, each 8 to 64 bytes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum NonceType {
    One(BytesType),
    More(Vec<BytesType>),
}
fn validate_nonce_size(b: &[u8]) -> Result<(), String> {
    if b.len() < 8 || b.len() > 64 {
        return Err(format!("Nonce must be 8..64 bytes, got {}", b.len()));
    }
    Ok(())
}

impl TryFrom<&Value> for NonceType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(k) => {
                validate_nonce_size(k)?;
                Ok(Self::One(BytesType::Bytes(k.clone())))
            }
            Value::Array(k) => {
                let mut items = Vec::new();
                for m in k.iter() {
                    match m.as_bytes() {
                        Some(b) => {
                            validate_nonce_size(b)?;
                            items.push(BytesType::Bytes(b.clone()));
                        }
                        None => {
                            return Err(
                                "Failed to parse array element as bytes in NonceType".to_string()
                            );
                        }
                    }
                }
                Ok(Self::More(items))
            }
            _ => Err("Failed to parse value as a NonceType".to_string()),
        }
    }
}

/// The `tagged-pkix-base64-key-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-pkix-base64-key-type = #6.554(tstr)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedPkixBase64KeyType = Required<String, 554>;

/// The `tagged-pkix-base64-cert-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-pkix-base64-cert-type = #6.555(tstr)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedPkixBase64CertType = Required<String, 555>;

/// The `tagged-pkix-base64-cert-path-type` is defined in [CoRIM Section 5.1.4.6].
///
/// ```text
/// tagged-pkix-base64-cert-path-type = #6.556(tstr)
/// ```
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
pub type TaggedPkixBase64CertPathType = Required<String, 556>;

/// ueid-type = bstr .size (7..33)
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
            Value::Bytes(k) => {
                if k.len() < 7 || k.len() > 33 {
                    return Err(alloc::format!(
                        "UeidType must be 7-33 bytes, got {}",
                        k.len()
                    ));
                }
                Ok(Self::Ueid(k.clone()))
            }
            _ => Err("Failed to parse value as a UeidType".to_string()),
        }
    }
}

/// The `tagged-ueid-type` is defined in [CoRIM Section 7.5].
///
/// ```text
/// tagged-ueid-type = #6.550(ueid-type)
/// ```
///
/// [CoRIM Section 7.5]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.5
pub type TaggedUeidType = Required<UeidType, 550>;

/// oid-type = bytes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum OidType {
    #[serde(with = "serde_bytes")]
    Oid(Vec<u8>),
}
/// The `tagged-oid-type` is defined in [CoRIM Section 7.6].
///
/// ```text
/// tagged-oid-type = #6.111(oid-type)
/// ```
///
/// [CoRIM Section 7.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.6
pub type TaggedOidTypeCbor = Required<OidType, 111>;

/// Alias for [`OidType`] used in non-CBOR (JSON) contexts where the tag is not applied.
#[allow(missing_docs)]
pub type TaggedOidType = OidType;

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
            Value::Bytes(k) => {
                if k.len() != 16 {
                    return Err(alloc::format!(
                        "UuidType must be exactly 16 bytes, got {}",
                        k.len()
                    ));
                }
                Ok(Self::Uuid(k.clone()))
            }
            _ => Err("Failed to parse value as a UuidType".to_string()),
        }
    }
}

/// The `tagged-uuid-type` is defined in [CoRIM Section 7.4].
///
/// ```text
/// tagged-uuid-type = #6.37(uuid-type)
/// ```
///
/// [CoRIM Section 7.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.4
pub type TaggedUuidType = Required<UuidType, 37>;

//pub type TaggedUriType = Required<Uri, 32>;
/// Alias for [`Uri`] used in non-CBOR (JSON) contexts where the tag is not applied.
#[allow(missing_docs)]
pub type TaggedUriType = Uri;

/// CBOR-encoded URI wrapped with tag 32, as defined in RFC 7049.
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
            Value::Tag(32, k) => match k.as_text() {
                Some(s) => Ok(Self::U(Required(s.to_string()))),
                None => Err("Expected text value inside tag 32 for TaggedUriTypeCbor".to_string()),
            },
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

/// A choice between a tagged URI or a tagged OID, used in non-CBOR (JSON) contexts.
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
            Value::Tag(32, k) => match k.as_text() {
                Some(s) => Ok(Self::U(s.to_string())),
                None => Err("Expected text value inside tag 32 for OidOrUri".to_string()),
            },
            Value::Tag(111, k) => match k.as_bytes() {
                Some(b) => Ok(Self::O(OidType::Oid(b.clone()))),
                None => Err("Expected bytes value inside tag 111 for OidOrUri".to_string()),
            },
            _ => Err("Failed to parse value as a OidOrUri".to_string()),
        }
    }
}

/// CBOR-encoded choice between a tagged URI (tag 32) or a tagged OID (tag 111).
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
            Value::Tag(32, k) => match k.as_text() {
                Some(s) => Ok(Self::U(TaggedUriTypeCbor::U(Required(s.to_string())))),
                None => Err("Expected text value inside tag 32 for OidOrUriCbor".to_string()),
            },
            Value::Tag(111, k) => match k.as_bytes() {
                Some(b) => Ok(Self::O(TaggedOidTypeCbor {
                    0: OidType::Oid(b.clone()),
                })),
                None => Err("Expected bytes value inside tag 111 for OidOrUriCbor".to_string()),
            },
            _ => Err("Failed to parse value as a OidOrUriCbor".to_string()),
        }
    }
}

/// A base64-encoded PKIX value represented as a text string.
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
/// Integer time value (`time-int`) used in non-CBOR (JSON) contexts where the tag is not applied.
#[allow(missing_docs)]
pub type Time = i64;

/// CBOR-encoded integer time value wrapped with tag 1, as defined in `time-int = #6.1(int)`.
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
            Value::Tag(_i, k) => {
                let integer = k
                    .as_integer()
                    .ok_or_else(|| "Expected integer value inside tag for TimeCbor".to_string())?;
                let val: i64 = integer
                    .try_into()
                    .map_err(|_| "Integer value out of range for TimeCbor".to_string())?;
                Ok(Self::T(Required(val)))
            }
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
/// coap-content-format = uint .le 65535
pub type CoapContentFormat = u16;

/// The `tagged-svn` type is defined in [CoRIM Section 5.1.4.5.4].
///
/// ```text
/// svn-type = uint
/// tagged-svn = #6.552(svn-type)
/// ```
///
/// [CoRIM Section 5.1.4.5.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.4
pub type TaggedSvn = Required<u64, 552>;

/// The `tagged-min-svn` type is defined in [CoRIM Section 5.1.4.5.4].
///
/// ```text
/// tagged-min-svn = #6.553(svn-type)
/// ```
///
/// [CoRIM Section 5.1.4.5.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.4
pub type TaggedMinSvn = Required<u64, 553>;

/// A choice between a text string and a byte string value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TextOrBinary {
    Text(String),
    #[serde(with = "serde_bytes")]
    Binary(Vec<u8>),
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

/// A choice between a byte string value and CBOR null.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum BinaryOrNil {
    #[serde(with = "serde_bytes")]
    Binary(Vec<u8>),
    Nil,
}

impl TryFrom<&Value> for BinaryOrNil {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Self::Nil),
            Value::Bytes(k) => Ok(Self::Binary(k.clone())),
            _ => Err("Failed to parse value as a TextOrBinary".to_string()),
        }
    }
}
impl TryFrom<Value> for BinaryOrNil {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Self::Nil),
            Value::Bytes(k) => Ok(Self::Binary(k)),
            _ => Err("Failed to parse value as a TextOrBinary".to_string()),
        }
    }
}

/// A PKIX certificate authority value represented as a byte string.
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

/// A choice between a text string and an integer value.
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
            Value::Integer(k) => {
                let val: i64 = Integer::try_into(*k)
                    .map_err(|_| "Integer value out of range for TextOrInt".to_string())?;
                Ok(Self::Int(val))
            }
            _ => Err("Failed to parse value as a TextOrInt".to_string()),
        }
    }
}
impl TryFrom<Value> for TextOrInt {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(k) => Ok(Self::Text(k)),
            Value::Integer(k) => {
                let val: i64 = Integer::try_into(k)
                    .map_err(|_| "Integer value out of range for TextOrInt".to_string())?;
                Ok(Self::Int(val))
            }
            _ => Err("Failed to parse value as a TextOrInt".to_string()),
        }
    }
}
