//! Choice-based types from the Concise Reference Integrity Manifest (CoRIM) spec
//! ([draft-ietf-rats-corim-10]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `tagged-coswid-type = #6.505(concise-swid-tag)` | [`TaggedCoswid`] / [`TaggedCoswidCbor`] |
//! | `tagged-concise-mid-tag = #6.506(concise-mid-tag)` | [`TaggedComid`] / [`TaggedComidCbor`] |
//! | `$concise-tag-type-choice` | [`ConciseTagTypeChoice`] |
//! | `$class-id-type-choice` | [`ClassIdTypeChoice`] / [`ClassIdTypeChoiceCbor`] |
//! | `$corim-id-type-choice` | [`CorimIdTypeChoice`] |
//! | `$corim-role-type-choice` | [`CorimRoleTypeChoice`] / [`CorimRoleTypeChoiceCbor`] |
//! | `$crypto-key-type-choice` | [`CryptoKeyTypeChoice`] / [`CryptoKeyTypeChoiceCbor`] |
//! | `$domain-type-choice` | [`DomainTypeChoice`] |
//! | `$entity-name-type-choice` | [`EntityNameTypeChoice`] |
//! | `$group-id-type-choice` | [`GroupIdTypeChoice`] |
//! | `$instance-id-type-choice` | [`InstanceIdTypeChoice`] |
//! | `$measured-element-type-choice` | [`MeasuredElementTypeChoice`] / [`MeasuredElementTypeChoiceCbor`] |
//! | `$profile-type-choice` | [`ProfileTypeChoice`] / [`ProfileTypeChoiceCbor`] |
//! | `$svn-type-choice` | [`SvnTypeChoice`] |
//! | `$tag-id-type-choice` | [`TagIdTypeChoice`] / [`TagIdTypeChoiceCbor`] |
//! | `$tag-rel-type-choice` | [`TagRelTypeChoice`] |
//! | `$tag-version-type` | [`TagVersionType`] |
//! | `$raw-value-type-choice` | [`RawValueTypeChoice`] / [`RawValueTypeChoiceCbor`] |
//! | `$int-range-type-choice` | [`IntRangeTypeChoice`] / [`IntRangeTypeChoiceCbor`] |
//!
//! [draft-ietf-rats-corim-10]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ciborium::value::{Integer, Value};
use common::*;
use core::fmt;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

use ciborium::tag::Required;

use crate::maps::*;
use coswid::maps::*;

/// $concise-tag-type-choice /= #6.505(bytes .cbor concise-swid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedCoswid(pub Required<ConciseSwidTag, 505>);

impl TryFrom<TaggedCoswidCbor> for TaggedCoswid {
    type Error = String;
    fn try_from(value: TaggedCoswidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.try_into()?)))
    }
}
impl TryFrom<&TaggedCoswidCbor> for TaggedCoswid {
    type Error = String;
    fn try_from(value: &TaggedCoswidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.clone().0.0.try_into()?)))
    }
}

/// $concise-tag-type-choice /= #6.506(bytes .cbor concise-mid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedComid(pub Required<ConciseMidTag, 506>);

impl TryFrom<TaggedComidCbor> for TaggedComid {
    type Error = String;
    fn try_from(value: TaggedComidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.try_into()?)))
    }
}
impl TryFrom<&TaggedComidCbor> for TaggedComid {
    type Error = String;
    fn try_from(value: &TaggedComidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.clone().try_into()?)))
    }
}

/// $concise-tag-type-choice /= #6.505(bytes .cbor concise-swid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedCoswidCbor(pub Required<ConciseSwidTagCbor, 505>);

impl TryFrom<TaggedCoswid> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: TaggedCoswid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.try_into()?)))
    }
}
impl TryFrom<&TaggedCoswid> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: &TaggedCoswid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.clone().try_into()?)))
    }
}

impl TryFrom<Value> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(505, b) => {
                let m = (*b).as_map().ok_or_else(|| {
                    "Expected map inside tag 505 for TaggedCoswidCbor".to_string()
                })?;
                Ok(Self(Required(m.try_into()?)))
            }
            _ => Err("Failed to parse value as TaggedCoswidCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(505, b) => {
                let m = (*b).as_map().ok_or_else(|| {
                    "Expected map inside tag 505 for TaggedCoswidCbor".to_string()
                })?;
                Ok(Self(Required(m.try_into()?)))
            }
            _ => Err("Failed to parse value as TaggedCoswidCbor".to_string()),
        }
    }
}

/// $concise-tag-type-choice /= #6.506(bytes .cbor concise-mid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedComidCbor(pub Required<ConciseMidTagCbor, 506>);

impl TryFrom<TaggedComid> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: TaggedComid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.try_into()?)))
    }
}
impl TryFrom<&TaggedComid> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: &TaggedComid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0.0.clone().try_into()?)))
    }
}

impl TryFrom<Value> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(506, b) => {
                let m = (*b)
                    .as_map()
                    .ok_or_else(|| "Expected map inside tag 506 for TaggedComidCbor".to_string())?;
                Ok(Self(Required(m.try_into()?)))
            }
            _ => Err("Failed to parse value as TaggedComidCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(506, b) => {
                let m = (*b)
                    .as_map()
                    .ok_or_else(|| "Expected map inside tag 506 for TaggedComidCbor".to_string())?;
                Ok(Self(Required(m.try_into()?)))
            }
            _ => Err("Failed to parse value as TaggedComidCbor".to_string()),
        }
    }
}

// todo: better to try to handle the types in this enum or just let it be bytes and handle it after parsing?
/// $concise-tag-type-choice /= #6.505(bytes .cbor concise-swid-tag)
/// $concise-tag-type-choice /= #6.506(bytes .cbor concise-mid-tag)
pub type ConciseTagTypeChoice = BytesType;
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[allow(missing_docs)]
// #[serde(untagged)]
// pub enum ConciseTagTypeChoice {
//     Coswid(TaggedCoswid),
//     Comid(TaggedComid),
// }
// impl TryFrom<ConciseTagTypeChoiceCbor> for ConciseTagTypeChoice {
//     type Error = String;
//     fn try_from(value: ConciseTagTypeChoiceCbor) -> Result<Self, Self::Error> {
//         match value {
//             ConciseTagTypeChoiceCbor::Coswid(b) => Ok(Self::Coswid(TaggedCoswid(Required(b.0.0.try_into().unwrap())))),
//             ConciseTagTypeChoiceCbor::Comid(b) => Ok(Self::Comid(TaggedComid(Required(b.0.0.try_into().unwrap())))),
//         }
//     }
// }
// impl TryFrom<&ConciseTagTypeChoiceCbor> for ConciseTagTypeChoice {
//     type Error = String;
//     fn try_from(value: &ConciseTagTypeChoiceCbor) -> Result<Self, Self::Error> {
//         match value {
//             ConciseTagTypeChoiceCbor::Coswid(b) => Ok(Self::Coswid(TaggedCoswid(Required(b.0.0.clone().try_into().unwrap())))),
//             ConciseTagTypeChoiceCbor::Comid(b) => Ok(Self::Comid(TaggedComid(Required(b.0.0.clone().try_into().unwrap())))),
//         }
//     }
// }
//
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[allow(missing_docs)]
// #[serde(untagged)]
// pub enum ConciseTagTypeChoiceCbor {
//     Coswid(TaggedCoswidCbor),
//     Comid(TaggedComidCbor),
// }
// impl TryFrom<ConciseTagTypeChoice> for ConciseTagTypeChoiceCbor {
//     type Error = String;
//     fn try_from(value: ConciseTagTypeChoice) -> Result<Self, Self::Error> {
//         match value {
//             ConciseTagTypeChoice::Coswid(b) => Ok(Self::Coswid(TaggedCoswidCbor(Required(b.0.0.try_into().unwrap())))),
//             ConciseTagTypeChoice::Comid(b) => Ok(Self::Comid(TaggedComidCbor(Required(b.0.0.try_into().unwrap())))),
//         }
//     }
// }
// impl TryFrom<&ConciseTagTypeChoice> for ConciseTagTypeChoiceCbor {
//     type Error = String;
//     fn try_from(value: &ConciseTagTypeChoice) -> Result<Self, Self::Error> {
//         match value {
//             ConciseTagTypeChoice::Coswid(b) => Ok(Self::Coswid(TaggedCoswidCbor(Required(b.0.0.clone().try_into().unwrap())))),
//             ConciseTagTypeChoice::Comid(b) => Ok(Self::Comid(TaggedComidCbor(Required(b.0.0.clone().try_into().unwrap())))),
//         }
//     }
// }
//
// impl TryFrom<Value> for ConciseTagTypeChoiceCbor {
//     type Error = String;
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         match &value {
//             Value::Bytes(b) => {
//                 let r : Result<TaggedComidCbor, _> = from_reader(b.as_slice());
//                 if r.is_ok() {
//                     return Ok(Self::Comid(r.unwrap()));
//                 }
//                 let r : Result<TaggedCoswidCbor, _> = from_reader(b.as_slice());
//                 if r.is_ok() {
//                     return Ok(Self::Coswid(r.unwrap()));
//                 }
//                 return Err(format!("Failed to parse value as ConciseTagTypeChoiceCbor: {:?}", value).to_string());
//             }
//             _ => {return Err(format!("Failed to parse value as ConciseTagTypeChoiceCbor: {:?}", value).to_string())}
//         }
//     }
// }
// impl TryFrom<&Value> for ConciseTagTypeChoiceCbor {
//     type Error = String;
//     fn try_from(value: &Value) -> Result<Self, Self::Error> {
//         match &value {
//             Value::Bytes(b) => {
//                 let r : Result<TaggedComidCbor, _> = from_reader(b.as_slice());
//                 if r.is_ok() {
//                     return Ok(Self::Comid(r.unwrap()));
//                 }
//                 let r : Result<TaggedCoswidCbor, _> = from_reader(b.as_slice());
//                 if r.is_ok() {
//                     return Ok(Self::Coswid(r.unwrap()));
//                 }
//                 return Err(format!("Failed to parse value as ConciseTagTypeChoiceCbor: {:?}", value).to_string());
//             }
//             _ => {return Err(format!("Failed to parse value as ConciseTagTypeChoiceCbor: {:?}", value).to_string())}
//         }
//     }
// }

/// The `class-id-type-choice` socket is defined in [CoRIM Section 5.1.4.2].
///
/// [CoRIM Section 5.1.4.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.2
///
/// ```text
/// $class-id-type-choice /= tagged-oid-type
/// $class-id-type-choice /= tagged-uuid-type
/// $class-id-type-choice /= tagged-bytes
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "value")]
pub enum ClassIdTypeChoice {
    oid(OidType),
    uuid(UuidType),
    bytes(BytesType),
}

impl TryFrom<ClassIdTypeChoiceCbor> for ClassIdTypeChoice {
    type Error = String;
    fn try_from(value: ClassIdTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoiceCbor::Oid(b) => Ok(Self::oid(b.0)),
            ClassIdTypeChoiceCbor::Uuid(b) => Ok(Self::uuid(b.0.clone())),
            ClassIdTypeChoiceCbor::Bytes(b) => Ok(Self::bytes(b.0)),
        }
    }
}
impl TryFrom<&ClassIdTypeChoiceCbor> for ClassIdTypeChoice {
    type Error = String;
    fn try_from(value: &ClassIdTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoiceCbor::Oid(b) => Ok(Self::oid(b.0.clone())),
            ClassIdTypeChoiceCbor::Uuid(b) => Ok(Self::uuid(b.0.clone())),
            ClassIdTypeChoiceCbor::Bytes(b) => Ok(Self::bytes(b.0.clone())),
        }
    }
}

/// CBOR-encodable counterpart of [`ClassIdTypeChoice`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ClassIdTypeChoiceCbor {
    Oid(TaggedOidTypeCbor),
    Uuid(TaggedUuidType),
    Bytes(TaggedBytes),
}
impl TryFrom<ClassIdTypeChoice> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: ClassIdTypeChoice) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoice::oid(b) => Ok(Self::Oid(TaggedOidTypeCbor { 0: b })),
            ClassIdTypeChoice::uuid(b) => Ok(Self::Uuid(TaggedUuidType { 0: b })),
            ClassIdTypeChoice::bytes(b) => Ok(Self::Bytes(TaggedBytes { 0: b })),
        }
    }
}
impl TryFrom<&ClassIdTypeChoice> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &ClassIdTypeChoice) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoice::oid(b) => Ok(Self::Oid(TaggedOidTypeCbor { 0: b.clone() })),
            ClassIdTypeChoice::uuid(b) => Ok(Self::Uuid(TaggedUuidType { 0: b.clone() })),
            ClassIdTypeChoice::bytes(b) => Ok(Self::Bytes(TaggedBytes { 0: b.clone() })),
        }
    }
}
impl TryFrom<Value> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse OID value as bytes".to_string()),
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse UUID value as bytes".to_string()),
                }),
            })),
            Value::Tag(560, b) => Ok(Self::Bytes(TaggedBytes {
                0: BytesType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse tagged bytes value".to_string()),
                }),
            })),
            _ => Err("Failed to parse value as a ClassIdTypeChoiceCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse OID value as bytes".to_string()),
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse UUID value as bytes".to_string()),
                }),
            })),
            Value::Tag(560, b) => Ok(Self::Bytes(TaggedBytes {
                0: BytesType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse tagged bytes value".to_string()),
                }),
            })),
            _ => Err("Failed to parse value as a ClassIdTypeChoiceCbor".to_string()),
        }
    }
}

/// The `corim-id-type-choice` socket is defined in [CoRIM Section 4.1.1].
///
/// ```text
/// $corim-id-type-choice /= tstr
/// $corim-id-type-choice /= uuid-type
/// ```
///
/// [CoRIM Section 4.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CorimIdTypeChoice {
    Str(String),
    Uuid(UuidType),
}
impl TryFrom<Value> for CorimIdTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        CorimIdTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for CorimIdTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Str(s.clone())),
            Value::Bytes(b) => Ok(Self::Uuid(UuidType(b.clone()))),
            _ => Err("Failed to parse value as a CorimIdTypeChoice".to_string()),
        }
    }
}

// impl<'de> _serde::Deserialize<'de> for CorimIdTypeChoice {
//     fn deserialize<__D>(
//         __deserializer: __D,
//     ) -> _serde::__private::Result<Self, __D::Error>
//         where
//             __D: _serde::Deserializer<'de>,
//     {
//         let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
//             __deserializer,
//         ) {
//             _serde::__private::Ok(__val) => __val,
//             _serde::__private::Err(__err) => {
//                 return _serde::__private::Err(__err);
//             }
//         };
//         if let _serde::__private::Ok(__ok)
//         = _serde::__private::Result::map(
//             <String as _serde::Deserialize>::deserialize(
//                 _serde::__private::de::ContentRefDeserializer::<
//                     __D::Error,
//                 >::new(&__content),
//             ),
//             CorimIdTypeChoice::Str,
//         ) {
//             return _serde::__private::Ok(__ok);
//         }
//         if let _serde::__private::Ok(__ok)
//         = _serde::__private::Result::map(
//             <UuidType as _serde::Deserialize>::deserialize(
//                 _serde::__private::de::ContentRefDeserializer::<
//                     __D::Error,
//                 >::new(&__content),
//             ),
//             CorimIdTypeChoice::Uuid,
//         ) {
//             return _serde::__private::Ok(__ok);
//         }
//         _serde::__private::Err(
//             _serde::de::Error::custom(
//                 "data did not match any variant of untagged enum CorimIdTypeChoice",
//             ),
//         )
//     }
// }

// Serde parses untagged enums as the first type that happens to parse. This implementation inspects
// the content type. Proc macro generated code from serde is in the comment above.
impl<'de> Deserialize<'de> for CorimIdTypeChoice {
    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        struct CorimIdVisitor;
        impl<'de> Visitor<'de> for CorimIdVisitor {
            type Value = CorimIdTypeChoice;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a string or byte array")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(CorimIdTypeChoice::Str(v.into()))
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CorimIdTypeChoice::Str(v))
            }
            fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(CorimIdTypeChoice::Uuid(UuidType(v.to_vec())))
            }
            fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(CorimIdTypeChoice::Uuid(UuidType(v)))
            }
        }
        __deserializer.deserialize_any(CorimIdVisitor)
    }
}

/// The `corim-role-type-choice` socket is defined in [CoRIM Section 4.1.5].
///
/// [CoRIM Section 4.1.5]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1.5
///
/// ```text
/// $corim-role-type-choice /= &(manifest-creator: 1)
/// $corim-role-type-choice /= &(manifest-signer: 2)
/// ```
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde_enum_str::Deserialize_enum_str,
    serde_enum_str::Serialize_enum_str,
)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum CorimRoleTypeChoice {
    #[serde(rename = "manifestCreator")]
    ManifestCreator,
    #[serde(rename = "manifestSigner")]
    ManifestSigner,
    #[serde(other)]
    other(String),
}

impl TryFrom<CorimRoleTypeChoice> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: CorimRoleTypeChoice) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoice::ManifestCreator => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestCreator))
            }
            CorimRoleTypeChoice::ManifestSigner => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestSigner))
            }
            CorimRoleTypeChoice::other(s) => Ok(Self::Extensions(match s.parse::<i8>() {
                Ok(i) => i,
                Err(e) => return Err(e.to_string()),
            })),
        }
    }
}

impl TryFrom<CorimRoleTypeChoiceCbor> for CorimRoleTypeChoice {
    type Error = String;
    fn try_from(value: CorimRoleTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoiceCbor::Known(v) => match v {
                CorimRoleTypeChoiceKnownCbor::ManifestCreator => Ok(Self::ManifestCreator),
                CorimRoleTypeChoiceKnownCbor::ManifestSigner => Ok(Self::ManifestSigner),
            },
            CorimRoleTypeChoiceCbor::Extensions(e) => Ok(Self::other(e.to_string())),
        }
    }
}

/// CBOR-encodable counterpart of [`CorimRoleTypeChoice`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CorimRoleTypeChoiceCbor {
    Known(CorimRoleTypeChoiceKnownCbor),
    Extensions(i8),
}
impl TryFrom<&CorimRoleTypeChoice> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &CorimRoleTypeChoice) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoice::ManifestCreator => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestCreator))
            }
            CorimRoleTypeChoice::ManifestSigner => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestSigner))
            }
            CorimRoleTypeChoice::other(s) => Ok(Self::Extensions(match s.parse::<i8>() {
                Ok(i) => i,
                Err(e) => return Err(e.to_string()),
            })),
        }
    }
}

impl TryFrom<&CorimRoleTypeChoiceCbor> for CorimRoleTypeChoice {
    type Error = String;
    fn try_from(value: &CorimRoleTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoiceCbor::Known(v) => match v {
                CorimRoleTypeChoiceKnownCbor::ManifestCreator => Ok(Self::ManifestCreator),
                CorimRoleTypeChoiceKnownCbor::ManifestSigner => Ok(Self::ManifestSigner),
            },
            CorimRoleTypeChoiceCbor::Extensions(e) => Ok(Self::other(e.to_string())),
        }
    }
}

impl TryFrom<Value> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => {
                if i.eq(&Integer::from(1)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestCreator))
                } else if i.eq(&Integer::from(2)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestSigner))
                } else {
                    Ok(Self::Extensions(match Integer::try_into(i) {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    }))
                }
            }
            _ => Err("Failed to parse value as an integer for CorimRoleTypeChoiceCbor".to_string()),
        }
    }
}

impl TryFrom<&Value> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => {
                if i.eq(&Integer::from(1)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestCreator))
                } else if i.eq(&Integer::from(2)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::ManifestSigner))
                } else {
                    Ok(Self::Extensions(match Integer::try_into(*i) {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    }))
                }
            }
            _ => Err("Failed to parse value as an integer for CorimRoleTypeChoiceCbor".to_string()),
        }
    }
}

/// CBOR-encodable enumeration of known `corim-role-type-choice` values.
#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[allow(missing_docs)]
#[repr(i8)]
pub enum CorimRoleTypeChoiceKnownCbor {
    ManifestCreator = 1,
    ManifestSigner = 2,
}

/// The `crypto-key-type-choice` socket is defined in [CoRIM Section 5.1.4.6].
///
/// [CoRIM Section 5.1.4.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.6
///
/// ```text
/// $crypto-key-type-choice /= tagged-pkix-base64-key-type       ; 554
/// $crypto-key-type-choice /= tagged-pkix-base64-cert-type      ; 555
/// $crypto-key-type-choice /= tagged-pkix-base64-cert-path-type ; 556
/// $crypto-key-type-choice /= tagged-key-thumbprint-type        ; 557
/// $crypto-key-type-choice /= tagged-cose-key-type              ; 558
/// $crypto-key-type-choice /= tagged-cert-thumbprint-type       ; 559
/// $crypto-key-type-choice /= tagged-bytes                      ; 560
/// $crypto-key-type-choice /= tagged-cert-path-thumbprint-type  ; 561
/// $crypto-key-type-choice /= tagged-pkix-asn1-der-cert-type    ; 562
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CryptoKeyTypeChoice {
    Key(TaggedPkixBase64KeyType),
    Cert(TaggedPkixBase64CertType),
    Path(TaggedPkixBase64CertPathType),
    KeyThumbprint(TaggedKeyThumbprintType),
    CoseKey(TaggedCoseKeyType),
    CertThumbprint(TaggedCertThumbprintType),
    Bytes(TaggedBytes),
    CertPathThumbprint(TaggedCertPathThumbprintType),
    DerCert(TaggedPkixAsn1DerCertType),
}

/// Type alias for CBOR form (same as non-CBOR form for this type)
pub type CryptoKeyTypeChoiceCbor = CryptoKeyTypeChoice;

impl TryFrom<&CryptoKeyTypeChoice> for CryptoKeyTypeChoice {
    type Error = String;
    fn try_from(value: &CryptoKeyTypeChoice) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}

fn parse_tagged_hash_entry(b: &Value) -> Result<arrays::HashEntry, String> {
    match b.as_array() {
        Some(arr) if arr.len() == 2 => {
            let alg = match arr[0].as_integer() {
                Some(i) => match i.try_into() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("Failed to parse hash alg id: {}", e)),
                },
                None => return Err("Failed to parse hash alg id as integer".to_string()),
            };
            let val = match arr[1].as_bytes() {
                Some(b) => b.clone(),
                None => return Err("Failed to parse hash value as bytes".to_string()),
            };
            Ok(arrays::HashEntry {
                hash_alg_id: alg,
                hash_value: val,
            })
        }
        _ => Err("Failed to parse hash-entry as a 2-element array".to_string()),
    }
}

impl TryFrom<Value> for CryptoKeyTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        CryptoKeyTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for CryptoKeyTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(554, b) => Ok(Self::Key(Required(match b.as_text() {
                Some(t) => t.to_string(),
                None => {
                    return Err("Failed to parse CryptoKeyTypeChoice tag 554 as text".to_string());
                }
            }))),
            Value::Tag(555, b) => Ok(Self::Cert(Required(match b.as_text() {
                Some(t) => t.to_string(),
                None => {
                    return Err("Failed to parse CryptoKeyTypeChoice tag 555 as text".to_string());
                }
            }))),
            Value::Tag(556, b) => Ok(Self::Path(Required(match b.as_text() {
                Some(t) => t.to_string(),
                None => {
                    return Err("Failed to parse CryptoKeyTypeChoice tag 556 as text".to_string());
                }
            }))),
            Value::Tag(557, b) => Ok(Self::KeyThumbprint(Required(parse_tagged_hash_entry(b)?))),
            Value::Tag(558, b) => Ok(Self::CoseKey(Required(BytesType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => {
                    return Err("Failed to parse CryptoKeyTypeChoice tag 558 as bytes".to_string());
                }
            })))),
            Value::Tag(559, b) => Ok(Self::CertThumbprint(Required(parse_tagged_hash_entry(b)?))),
            Value::Tag(560, b) => Ok(Self::Bytes(Required(BytesType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => {
                    return Err("Failed to parse CryptoKeyTypeChoice tag 560 as bytes".to_string());
                }
            })))),
            Value::Tag(561, b) => Ok(Self::CertPathThumbprint(Required(parse_tagged_hash_entry(
                b,
            )?))),
            Value::Tag(562, b) => Ok(Self::DerCert(Required(BytesType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => {
                    return Err("Failed to parse CryptoKeyTypeChoice tag 562 as bytes".to_string());
                }
            })))),
            _ => Err("Failed to parse value as a CryptoKeyTypeChoice".to_string()),
        }
    }
}

/// The `domain-type-choice` socket is defined in [CoRIM Section 5.1.11].
///
/// ```text
/// $domain-type-choice /= uint
/// $domain-type-choice /= text
/// $domain-type-choice /= tagged-uuid-type
/// ```
///
/// [CoRIM Section 5.1.11]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.11
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum DomainTypeChoice {
    U64(u64),
    Text(String),
    Uuid(TaggedUuidType),
}
impl TryFrom<Value> for DomainTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(Self::U64(match Integer::try_into(i) {
                Ok(i) => i,
                Err(e) => return Err(e.to_string()),
            })),
            Value::Text(s) => Ok(Self::Text(s)),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err("Failed to parse UUID value as a DomainTypeChoice".to_string());
                    }
                }),
            })),
            _ => Err("Failed to parse value as a DomainTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for DomainTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(Self::U64(match Integer::try_into(*i) {
                Ok(b) => b,
                Err(e) => return Err(e.to_string()),
            })),
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse UUID as a DomainTypeChoice".to_string()),
                }),
            })),
            _ => Err("Failed to parse value as a DomainTypeChoice".to_string()),
        }
    }
}

/// The `entity-name-type-choice` socket is defined in [CoRIM Section 7.2].
///
/// [CoRIM Section 7.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.2
///
/// ```text
/// $entity-name-type-choice /= text
/// $entity-name-type-choice /= tagged-oid-type
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EntityNameTypeChoice {
    Text(String),
    Oid(TaggedOidTypeCbor),
}
impl TryFrom<&Value> for EntityNameTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(EntityNameTypeChoice::Text(s.to_string())),
            Value::Tag(111, b) => Ok(EntityNameTypeChoice::Oid(TaggedOidTypeCbor {
                0: OidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse OID in EntityNameTypeChoice".to_string()),
                }),
            })),
            _ => Err("Failed to parse value as a EntityNameTypeChoice".to_string()),
        }
    }
}

/// $group-id-type-choice /= tagged-uuid-type / tagged-bytes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum GroupIdTypeChoice {
    Uuid(TaggedUuidType),
    Bytes(TaggedBytes),
}
impl TryFrom<Value> for GroupIdTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        GroupIdTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for GroupIdTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err("Failed to parse UUID value as a GroupIdTypeChoice".to_string());
                    }
                }),
            })),
            Value::Tag(560, b) => Ok(Self::Bytes(TaggedBytes {
                0: BytesType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse tagged bytes value as a GroupIdTypeChoice".to_string()
                        );
                    }
                }),
            })),
            _ => Err("Failed to parse value as a GroupIdTypeChoice".to_string()),
        }
    }
}

/// The `instance-id-type-choice` socket is defined in [CoRIM Section 5.1.4.3].
///
/// [CoRIM Section 5.1.4.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.3
///
/// Includes tagged-ueid-type, tagged-uuid-type, and all crypto-key-type-choice variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum InstanceIdTypeChoice {
    Ueid(TaggedUeidType),
    Uuid(TaggedUuidType),
    CryptoKey(CryptoKeyTypeChoice),
}
impl TryFrom<Value> for InstanceIdTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        InstanceIdTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for InstanceIdTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as a InstanceIdTypeChoice".to_string()
                        );
                    }
                }),
            })),
            Value::Tag(550, b) => Ok(Self::Ueid(TaggedUeidType {
                0: UeidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UEID value as a InstanceIdTypeChoice".to_string()
                        );
                    }
                }),
            })),
            Value::Tag(t, _) if (554..=562).contains(t) => {
                Ok(Self::CryptoKey(CryptoKeyTypeChoice::try_from(value)?))
            }
            _ => Err("Failed to parse value as a InstanceIdTypeChoice".to_string()),
        }
    }
}

/// The `measured-element-type-choice` socket is defined in [CoRIM Section 5.1.4.5.1].
///
/// ```text
/// $measured-element-type-choice /= tagged-oid-type
/// $measured-element-type-choice /= tagged-uuid-type
/// $measured-element-type-choice /= uint
/// $measured-element-type-choice /= text
/// ```
///
/// The `Other` variant accepts any CBOR tagged value as a [`Tuple`] with `key` set to
/// `Value::Integer(Integer::from(tag))` and `value` set to the tag content. This supports
/// spec-defined extensibility via the CDDL socket (`$measured-element-type-choice /=`).
///
/// [CoRIM Section 5.1.4.5.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.1
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum MeasuredElementTypeChoice {
    Oid(TaggedOidType),
    Uuid(TaggedUuidType),
    Uint(u64),
    Text(String),
    Other(Tuple),
}

impl TryFrom<MeasuredElementTypeChoiceCbor> for MeasuredElementTypeChoice {
    type Error = String;
    fn try_from(value: MeasuredElementTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            MeasuredElementTypeChoiceCbor::Oid(b) => Ok(Self::Oid(b.0)),
            MeasuredElementTypeChoiceCbor::Uuid(b) => Ok(Self::Uuid(Required(b.0))),
            MeasuredElementTypeChoiceCbor::Uint(v) => Ok(Self::Uint(v)),
            MeasuredElementTypeChoiceCbor::Text(s) => Ok(Self::Text(s)),
            MeasuredElementTypeChoiceCbor::Other(b) => match Tuple::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<&MeasuredElementTypeChoiceCbor> for MeasuredElementTypeChoice {
    type Error = String;
    fn try_from(value: &MeasuredElementTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            MeasuredElementTypeChoiceCbor::Oid(b) => Ok(Self::Oid(b.0.clone())),
            MeasuredElementTypeChoiceCbor::Uuid(b) => Ok(Self::Uuid(Required(b.0.clone()))),
            MeasuredElementTypeChoiceCbor::Uint(v) => Ok(Self::Uint(*v)),
            MeasuredElementTypeChoiceCbor::Text(s) => Ok(Self::Text(s.clone())),
            MeasuredElementTypeChoiceCbor::Other(b) => match Tuple::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<Value> for MeasuredElementTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        MeasuredElementTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for MeasuredElementTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(OidType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => {
                    return Err(
                        "Failed to parse OID value as a MeasuredElementTypeChoice".to_string()
                    );
                }
            }))),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as a MeasuredElementTypeChoice".to_string()
                        );
                    }
                }),
            })),
            Value::Integer(i) => Ok(Self::Uint(match (*i).try_into() {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Failed to parse uint in MeasuredElementTypeChoice: {}",
                        e
                    ));
                }
            })),
            Value::Text(s) => Ok(Self::Text(s.clone())),
            _ => Err(format!(
                "Failed to parse MeasuredElementTypeChoice from value: {:?}",
                value
            )),
        }
    }
}

/// CBOR-encodable counterpart of [`MeasuredElementTypeChoice`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum MeasuredElementTypeChoiceCbor {
    Oid(TaggedOidTypeCbor),
    Uuid(TaggedUuidType),
    Uint(u64),
    Text(String),
    Other(TupleCbor),
}
impl TryFrom<MeasuredElementTypeChoice> for MeasuredElementTypeChoiceCbor {
    type Error = String;
    fn try_from(value: MeasuredElementTypeChoice) -> Result<Self, Self::Error> {
        match value {
            MeasuredElementTypeChoice::Oid(b) => Ok(Self::Oid(Required(b))),
            MeasuredElementTypeChoice::Uuid(b) => Ok(Self::Uuid(Required(b.0))),
            MeasuredElementTypeChoice::Uint(v) => Ok(Self::Uint(v)),
            MeasuredElementTypeChoice::Text(s) => Ok(Self::Text(s)),
            MeasuredElementTypeChoice::Other(b) => match TupleCbor::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<&MeasuredElementTypeChoice> for MeasuredElementTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &MeasuredElementTypeChoice) -> Result<Self, Self::Error> {
        match value {
            MeasuredElementTypeChoice::Oid(b) => Ok(Self::Oid(Required(b.clone()))),
            MeasuredElementTypeChoice::Uuid(b) => Ok(Self::Uuid(Required(b.0.clone()))),
            MeasuredElementTypeChoice::Uint(v) => Ok(Self::Uint(*v)),
            MeasuredElementTypeChoice::Text(s) => Ok(Self::Text(s.clone())),
            MeasuredElementTypeChoice::Other(b) => match TupleCbor::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<Value> for MeasuredElementTypeChoiceCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        MeasuredElementTypeChoiceCbor::try_from(&value)
    }
}
impl TryFrom<&Value> for MeasuredElementTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err("Failed to parse OID value as MeasuredElementTypeChoiceCbor"
                            .to_string());
                    }
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as MeasuredElementTypeChoiceCbor"
                                .to_string(),
                        );
                    }
                }),
            })),
            Value::Integer(i) => Ok(Self::Uint(match (*i).try_into() {
                Ok(v) => v,
                Err(e) => return Err(format!("Failed to parse uint: {}", e)),
            })),
            Value::Text(s) => Ok(Self::Text(s.clone())),
            Value::Tag(t, b) => Ok(Self::Other(TupleCbor {
                key: Value::Integer(Integer::from(*t)),
                value: *b.clone(),
            })),
            _ => Err("Failed to parse value as MeasuredElementTypeChoiceCbor".to_string()),
        }
    }
}

/// The `profile-type-choice` socket is defined in [CoRIM Section 4.1.4].
///
/// ```text
/// profile-type-choice = uri / tagged-oid-type
/// ```
///
/// [CoRIM Section 4.1.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1.4
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ProfileTypeChoice {
    Uri(Uri),
    Oid(TaggedOidType),
    Oid2(OidType),
    Other(Tuple),
}
impl TryFrom<ProfileTypeChoiceCbor> for ProfileTypeChoice {
    type Error = String;
    fn try_from(value: ProfileTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            ProfileTypeChoiceCbor::Uri(s) => Ok(Self::Uri(s)),
            ProfileTypeChoiceCbor::Oid(b) => Ok(Self::Oid(match b {
                Required(b) => b,
            })),
            ProfileTypeChoiceCbor::Oid2(b) => Ok(Self::Oid2(b)),
            ProfileTypeChoiceCbor::Other(b) => match Tuple::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<&ProfileTypeChoiceCbor> for ProfileTypeChoice {
    type Error = String;
    fn try_from(value: &ProfileTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            ProfileTypeChoiceCbor::Uri(s) => Ok(Self::Uri(s.clone())),
            ProfileTypeChoiceCbor::Oid(b) => Ok(Self::Oid(match b {
                Required(b) => b.clone(),
            })),
            ProfileTypeChoiceCbor::Oid2(b) => Ok(Self::Oid2(b.clone())),
            ProfileTypeChoiceCbor::Other(b) => match Tuple::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<Value> for ProfileTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Uri(s)),
            Value::Bytes(s) => Ok(Self::Oid2(OidType(s))),
            Value::Tag(111, b) => Ok(Self::Oid(OidType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => return Err("Failed to parse OID value as an ProfileTypeChoice".to_string()),
            }))),
            _ => Err("Failed to parse value as a ProfileTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for ProfileTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Uri(s.clone())),
            Value::Bytes(s) => Ok(Self::Oid2(OidType(s.clone()))),
            Value::Tag(111, b) => Ok(Self::Oid(OidType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => return Err("Failed to parse OID value as an ProfileTypeChoice".to_string()),
            }))),
            _ => Err("Failed to parse value as a ProfileTypeChoice".to_string()),
        }
    }
}

/// CBOR-encodable counterpart of [`ProfileTypeChoice`].
//todo the untagged OID field was added to interop with corim repo artifacts (and it raises questions re: use of Tuple for extensibility)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ProfileTypeChoiceCbor {
    Uri(Uri),
    Oid(TaggedOidTypeCbor),
    Oid2(OidType),
    Other(TupleCbor),
}
impl TryFrom<ProfileTypeChoice> for ProfileTypeChoiceCbor {
    type Error = String;
    fn try_from(value: ProfileTypeChoice) -> Result<Self, Self::Error> {
        match value {
            ProfileTypeChoice::Uri(s) => Ok(Self::Uri(s)),
            ProfileTypeChoice::Oid(b) => Ok(Self::Oid(TaggedOidTypeCbor { 0: b })),
            ProfileTypeChoice::Oid2(b) => Ok(Self::Oid2(b)),
            ProfileTypeChoice::Other(b) => match TupleCbor::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<&ProfileTypeChoice> for ProfileTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &ProfileTypeChoice) -> Result<Self, Self::Error> {
        match value {
            ProfileTypeChoice::Uri(s) => Ok(Self::Uri(s.clone())),
            ProfileTypeChoice::Oid(b) => Ok(Self::Oid(TaggedOidTypeCbor { 0: b.clone() })),
            ProfileTypeChoice::Oid2(b) => Ok(Self::Oid2(b.clone())),
            ProfileTypeChoice::Other(b) => match TupleCbor::try_from(b) {
                Ok(v) => Ok(Self::Other(v)),
                Err(e) => Err(e),
            },
        }
    }
}
impl TryFrom<Value> for ProfileTypeChoiceCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Uri(s)),
            Value::Bytes(s) => Ok(Self::Oid2(OidType(s))),
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse OID value as an ProfileTypeChoiceCbor".to_string()
                        );
                    }
                }),
            })),
            Value::Tag(t, b) => Ok(Self::Other(TupleCbor {
                key: Value::Integer(Integer::from(t)),
                value: *b,
            })),
            _ => Err("Failed to parse value as an ProfileTypeChoiceCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for ProfileTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Uri(s.clone())),
            Value::Bytes(s) => Ok(Self::Oid2(OidType(s.clone()))),
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse OID value as an ProfileTypeChoiceCbor".to_string()
                        );
                    }
                }),
            })),
            Value::Tag(t, b) => Ok(Self::Other(TupleCbor {
                key: Value::Integer(Integer::from(*t)),
                value: *b.clone(),
            })),
            _ => Err("Failed to parse value as an ProfileTypeChoiceCbor".to_string()),
        }
    }
}

/// The `svn-type-choice` socket is defined in [CoRIM Section 5.1.4.5.4].
///
/// ```text
/// svn-type-choice = tagged-svn / tagged-min-svn
/// ```
///
/// [CoRIM Section 5.1.4.5.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.4
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SvnTypeChoice {
    TaggedSvn(TaggedSvn),
    TaggedMinSvn(TaggedMinSvn),
}
impl TryFrom<Value> for SvnTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(552, b) => Ok(Self::TaggedSvn(TaggedSvn {
                0: match b.as_integer() {
                    Some(i) => match i.try_into() {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    },
                    None => {
                        return Err(
                            "Failed to parse tagged SVN value as an SvnTypeChoice".to_string()
                        );
                    }
                },
            })),
            Value::Tag(553, b) => Ok(Self::TaggedMinSvn(TaggedMinSvn {
                0: match b.as_integer() {
                    Some(i) => match i.try_into() {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    },
                    None => {
                        return Err(
                            "Failed to parse tagged min SVN value as an SvnTypeChoice".to_string()
                        );
                    }
                },
            })),
            _ => Err("Failed to parse value as an SvnTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for SvnTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(552, b) => Ok(Self::TaggedSvn(TaggedSvn {
                0: match b.as_integer() {
                    Some(i) => match i.try_into() {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    },
                    None => {
                        return Err(
                            "Failed to parse tagged SVN value as an SvnTypeChoice".to_string()
                        );
                    }
                },
            })),
            Value::Tag(553, b) => Ok(Self::TaggedMinSvn(TaggedMinSvn {
                0: match b.as_integer() {
                    Some(i) => match i.try_into() {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    },
                    None => {
                        return Err(
                            "Failed to parse tagged min SVN value as an SvnTypeChoice".to_string()
                        );
                    }
                },
            })),
            _ => Err("Failed to parse value as an SvnTypeChoice".to_string()),
        }
    }
}

/// The `tag-id-type-choice` socket is defined in [CoRIM Section 5.1.1.1].
///
/// ```text
/// $tag-id-type-choice /= tstr
/// $tag-id-type-choice /= uuid-type
/// ```
///
/// [CoRIM Section 5.1.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.1.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagIdTypeChoice {
    Str(String),
    Uuid(UuidType),
}

impl TryFrom<TagIdTypeChoiceCbor> for TagIdTypeChoice {
    type Error = String;
    fn try_from(value: TagIdTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            TagIdTypeChoiceCbor::Str(s) => Ok(Self::Str(s)),
            TagIdTypeChoiceCbor::Uuid(b) => Ok(Self::Uuid(b)),
        }
    }
}
impl TryFrom<&TagIdTypeChoiceCbor> for TagIdTypeChoice {
    type Error = String;
    fn try_from(value: &TagIdTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            TagIdTypeChoiceCbor::Str(s) => Ok(Self::Str(s.clone())),
            TagIdTypeChoiceCbor::Uuid(b) => Ok(Self::Uuid(b.clone())),
        }
    }
}

/// CBOR-encodable counterpart of [`TagIdTypeChoice`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagIdTypeChoiceCbor {
    Str(String),
    Uuid(UuidType),
}
impl TryFrom<TagIdTypeChoice> for TagIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: TagIdTypeChoice) -> Result<Self, Self::Error> {
        match value {
            TagIdTypeChoice::Str(s) => Ok(Self::Str(s)),
            TagIdTypeChoice::Uuid(b) => Ok(Self::Uuid(b)),
        }
    }
}
impl TryFrom<&TagIdTypeChoice> for TagIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &TagIdTypeChoice) -> Result<Self, Self::Error> {
        match value {
            TagIdTypeChoice::Str(s) => Ok(Self::Str(s.clone())),
            TagIdTypeChoice::Uuid(b) => Ok(Self::Uuid(b.clone())),
        }
    }
}
impl TryFrom<Value> for TagIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
impl TryFrom<&Value> for TagIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Str(s.clone())),
            Value::Bytes(b) => Ok(Self::Uuid(UuidType(b.clone()))),
            _ => Err("Failed to parse value as a TagIdTypeChoiceCbor".to_string()),
        }
    }
}
// Serde does not parse untagged enums properly (it just parses as the first type)
impl<'de> Deserialize<'de> for TagIdTypeChoiceCbor {
    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        struct TagIdVisitor;
        impl<'de> Visitor<'de> for TagIdVisitor {
            type Value = TagIdTypeChoiceCbor;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a string or byte array")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(TagIdTypeChoiceCbor::Str(v.into()))
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TagIdTypeChoiceCbor::Str(v))
            }
            fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(TagIdTypeChoiceCbor::Uuid(UuidType(v.to_vec())))
            }
            fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(TagIdTypeChoiceCbor::Uuid(UuidType(v)))
            }
        }
        __deserializer.deserialize_any(TagIdVisitor)
    }
}

/// The `tag-rel-type-choice` socket is defined in [CoRIM Section 5.1.3].
///
/// ```text
/// $tag-rel-type-choice /= &(supplements: 0)
/// $tag-rel-type-choice /= &(replaces: 1)
/// ```
///
/// [CoRIM Section 5.1.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagRelTypeChoice {
    Known(TagRelTypeChoiceKnown),
    Extensions(i8),
}

/// Enumeration of known `tag-rel-type-choice` values (supplements, replaces).
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[allow(missing_docs)]
pub enum TagRelTypeChoiceKnown {
    Supplements = 0,
    Replaces = 1,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for TagRelTypeChoiceKnown {
        fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                TagRelTypeChoiceKnown::Supplements => __serializer.serialize_i8(0),
                TagRelTypeChoiceKnown::Replaces => __serializer.serialize_i8(1),
            }
        }
    }
};

impl TryFrom<Value> for TagRelTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => {
                if i.eq(&Integer::from(0)) {
                    Ok(Self::Known(TagRelTypeChoiceKnown::Supplements))
                } else if i.eq(&Integer::from(1)) {
                    Ok(Self::Known(TagRelTypeChoiceKnown::Replaces))
                } else {
                    Ok(Self::Extensions(match Integer::try_into(i) {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    }))
                }
            }
            _ => Err("Failed to parse TagRelTypeChoice as an integer".to_string()),
        }
    }
}
impl TryFrom<&Value> for TagRelTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => {
                if i.eq(&Integer::from(0)) {
                    Ok(Self::Known(TagRelTypeChoiceKnown::Supplements))
                } else if i.eq(&Integer::from(1)) {
                    Ok(Self::Known(TagRelTypeChoiceKnown::Replaces))
                } else {
                    Ok(Self::Extensions(match Integer::try_into(*i) {
                        Ok(i) => i,
                        Err(e) => return Err(e.to_string()),
                    }))
                }
            }
            _ => Err("Failed to parse TagRelTypeChoice as an integer".to_string()),
        }
    }
}

// todo defaults
/// The `tag-version-type` socket is defined in [CoRIM Section 5.1.1.2].
///
/// ```text
/// tag-version-type = uint .default 0
/// ```
///
/// [CoRIM Section 5.1.1.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.1.2
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagVersionType {
    U64(u64),
}

impl TryFrom<Value> for TagVersionType {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(TagVersionType::U64(match Integer::try_into(i) {
                Ok(i) => i,
                Err(e) => return Err(e.to_string()),
            })),
            _ => Err("Failed to parse TagVersionType as an integer".to_string()),
        }
    }
}
impl TryFrom<&Value> for TagVersionType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(TagVersionType::U64(match Integer::try_into(*i) {
                Ok(i) => i,
                Err(e) => return Err(e.to_string()),
            })),
            _ => Err("Failed to parse TagVersionType as an integer".to_string()),
        }
    }
}

/// The `raw-value-type-choice` is defined in [CoRIM Section 5.1.4.5.6].
///
/// [CoRIM Section 5.1.4.5.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.6
///
/// ```text
/// $raw-value-type-choice /= tagged-bytes          ; #6.560
/// $raw-value-type-choice /= tagged-masked-raw-value ; #6.563
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum RawValueTypeChoice {
    Bytes(TaggedBytes),
    MaskedRawValue(TaggedMaskedRawValue),
}
/// Type alias for CBOR form (same as non-CBOR form for this type)
pub type RawValueTypeChoiceCbor = RawValueTypeChoice;

impl TryFrom<&RawValueTypeChoice> for RawValueTypeChoice {
    type Error = String;
    fn try_from(value: &RawValueTypeChoice) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}
impl TryFrom<Value> for RawValueTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        RawValueTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for RawValueTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(560, b) => Ok(Self::Bytes(Required(BytesType(match b.as_bytes() {
                Some(b) => b.clone(),
                None => return Err("Failed to parse tag 560 as bytes".to_string()),
            })))),
            Value::Tag(563, b) => {
                let arr = b.as_array().ok_or("Failed to parse tag 563 as array")?;
                if arr.len() != 2 {
                    return Err("MaskedRawValue must be a 2-element array".to_string());
                }
                let v = arr[0]
                    .as_bytes()
                    .ok_or("Failed to parse masked raw value")?
                    .clone();
                let m = arr[1]
                    .as_bytes()
                    .ok_or("Failed to parse mask value")?
                    .clone();
                Ok(Self::MaskedRawValue(Required(arrays::MaskedRawValueCbor {
                    value: v,
                    mask: m,
                })))
            }
            _ => Err("Failed to parse value as RawValueTypeChoice".to_string()),
        }
    }
}

/// The `int-range-type-choice` is defined in [CoRIM Section 5.1.4.8].
///
/// [CoRIM Section 5.1.4.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.8
///
/// ```text
/// $int-range-type-choice /= int
/// $int-range-type-choice /= tagged-int-range ; #6.564
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum IntRangeTypeChoice {
    Int(i64),
    Range(TaggedIntRange),
}
/// Type alias for CBOR form (same as non-CBOR form for this type)
pub type IntRangeTypeChoiceCbor = IntRangeTypeChoice;

impl TryFrom<&IntRangeTypeChoice> for IntRangeTypeChoice {
    type Error = String;
    fn try_from(value: &IntRangeTypeChoice) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}
impl TryFrom<Value> for IntRangeTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        IntRangeTypeChoice::try_from(&value)
    }
}
impl TryFrom<&Value> for IntRangeTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(Self::Int(match (*i).try_into() {
                Ok(v) => v,
                Err(e) => return Err(format!("Failed to parse int in IntRangeTypeChoice: {}", e)),
            })),
            Value::Tag(564, b) => {
                let arr = b.as_array().ok_or("Failed to parse tag 564 as array")?;
                if arr.len() != 2 {
                    return Err("IntRange must be a 2-element array".to_string());
                }
                let min: i64 = arr[0]
                    .as_integer()
                    .ok_or("Failed to parse min")?
                    .try_into()
                    .map_err(|e| format!("Failed to parse min: {}", e))?;
                let max: i64 = arr[1]
                    .as_integer()
                    .ok_or("Failed to parse max")?
                    .try_into()
                    .map_err(|e| format!("Failed to parse max: {}", e))?;
                Ok(Self::Range(Required(arrays::IntRangeCbor { min, max })))
            }
            _ => Err("Failed to parse value as IntRangeTypeChoice".to_string()),
        }
    }
}
