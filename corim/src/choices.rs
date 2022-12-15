//! Choice-based structs from the Concise Reference Integrity Manifest (CoRIM) spec

use alloc::format;
use alloc::string::{String, ToString};
use ciborium::value::{Integer, Value};
use common::*;
use serde::__private::de::Content;
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
        Ok(Self(Required(value.0 .0.try_into().unwrap())))
    }
}
impl TryFrom<&TaggedCoswidCbor> for TaggedCoswid {
    type Error = String;
    fn try_from(value: &TaggedCoswidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.clone().0 .0.try_into().unwrap())))
    }
}

/// $concise-tag-type-choice /= #6.506(bytes .cbor concise-mid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedComid(pub Required<ConciseMidTag, 506>);

impl TryFrom<TaggedComidCbor> for TaggedComid {
    type Error = String;
    fn try_from(value: TaggedComidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0 .0.try_into().unwrap())))
    }
}
impl TryFrom<&TaggedComidCbor> for TaggedComid {
    type Error = String;
    fn try_from(value: &TaggedComidCbor) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0 .0.clone().try_into().unwrap())))
    }
}

/// $concise-tag-type-choice /= #6.505(bytes .cbor concise-swid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedCoswidCbor(pub Required<ConciseSwidTagCbor, 505>);

impl TryFrom<TaggedCoswid> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: TaggedCoswid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0 .0.try_into().unwrap())))
    }
}
impl TryFrom<&TaggedCoswid> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: &TaggedCoswid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0 .0.clone().try_into().unwrap())))
    }
}

impl TryFrom<Value> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(505, b) => Ok(Self(Required((*b).as_map().unwrap().try_into().unwrap()))),
            _ => Err("Failed to parse value as TaggedComidCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for TaggedCoswidCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(505, b) => Ok(Self(Required((*b).as_map().unwrap().try_into().unwrap()))),
            _ => Err("Failed to parse value as TaggedComidCbor".to_string()),
        }
    }
}

/// $concise-tag-type-choice /= #6.506(bytes .cbor concise-mid-tag)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedComidCbor(pub Required<ConciseMidTagCbor, 506>);

impl TryFrom<TaggedComid> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: TaggedComid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0 .0.try_into().unwrap())))
    }
}
impl TryFrom<&TaggedComid> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: &TaggedComid) -> Result<Self, Self::Error> {
        Ok(Self(Required(value.0 .0.clone().try_into().unwrap())))
    }
}

impl TryFrom<Value> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(506, b) => Ok(Self(Required((*b).as_map().unwrap().try_into().unwrap()))),
            _ => Err("Failed to parse value as TaggedComidCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for TaggedComidCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(506, b) => Ok(Self(Required((*b).as_map().unwrap().try_into().unwrap()))),
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

/// The `class-id-type-choice` socket is defined in [CoRIM Section 3.1.4.1.2].
///
/// ```text
/// $class-id-type-choice /= tagged-oid-type
/// $class-id-type-choice /= tagged-uuid-type
/// $class-id-type-choice /= tagged-int-type
/// ```
///
/// [CoRIM Section 3.1.4.1.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.2
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "value")]
pub enum ClassIdTypeChoice {
    oid(OidType),
    uuid(UuidType),
    int(IntType),
}

impl TryFrom<ClassIdTypeChoiceCbor> for ClassIdTypeChoice {
    type Error = String;
    fn try_from(value: ClassIdTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoiceCbor::Oid(b) => Ok(Self::oid(b.0)),
            ClassIdTypeChoiceCbor::Uuid(b) => Ok(Self::uuid(match &b.0 {
                UuidType::Uuid(v) => common::UuidType::Uuid(v.clone()),
            })),
            ClassIdTypeChoiceCbor::Int(b) => Ok(Self::int(b.0)),
            ClassIdTypeChoiceCbor::Int2(b) => Ok(Self::int(b.0)),
        }
    }
}
impl TryFrom<&ClassIdTypeChoiceCbor> for ClassIdTypeChoice {
    type Error = String;
    fn try_from(value: &ClassIdTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoiceCbor::Oid(b) => Ok(Self::oid(b.0.clone())),
            ClassIdTypeChoiceCbor::Uuid(b) => Ok(Self::uuid(match &b.0 {
                UuidType::Uuid(v) => common::UuidType::Uuid(v.clone()),
            })),
            ClassIdTypeChoiceCbor::Int(b) => Ok(Self::int(b.0.clone())),
            ClassIdTypeChoiceCbor::Int2(b) => Ok(Self::int(b.0.clone())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ClassIdTypeChoiceCbor {
    Oid(TaggedOidTypeCbor),
    Uuid(TaggedUuidType),
    Int(TaggedIntType),
    Int2(TaggedIntType2),
}
impl TryFrom<ClassIdTypeChoice> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: ClassIdTypeChoice) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoice::oid(b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b {
                    TaggedOidType::Oid(v) => v,
                }),
            })),
            ClassIdTypeChoice::uuid(b) => Ok(Self::Uuid(TaggedUuidType { 0: b })),
            ClassIdTypeChoice::int(b) => Ok(Self::Int(TaggedIntType { 0: b })),
        }
    }
}
impl TryFrom<&ClassIdTypeChoice> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &ClassIdTypeChoice) -> Result<Self, Self::Error> {
        match value {
            ClassIdTypeChoice::oid(b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b {
                    TaggedOidType::Oid(v) => v.clone(),
                }),
            })),
            ClassIdTypeChoice::uuid(b) => Ok(Self::Uuid(TaggedUuidType { 0: b.clone() })),
            ClassIdTypeChoice::int(b) => Ok(Self::Int(TaggedIntType { 0: b.clone() })),
        }
    }
}
//todo the cocli tests use tag 600 here
impl TryFrom<Value> for ClassIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse OID value as bytes".to_string()),
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse UUID value as bytes".to_string()),
                }),
            })),
            Value::Tag(551, b) => Ok(Self::Int(TaggedIntType {
                0: IntType::Int(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse int value as bytes".to_string()),
                }),
            })),
            Value::Tag(600, b) => Ok(Self::Int2(TaggedIntType2 {
                0: IntType::Int(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse int value as bytes".to_string()),
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
                0: OidType::Oid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse OID value as bytes".to_string()),
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse UUID value as bytes".to_string()),
                }),
            })),
            Value::Tag(551, b) => Ok(Self::Int(TaggedIntType {
                0: IntType::Int(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse int value as bytes".to_string()),
                }),
            })),
            Value::Tag(600, b) => Ok(Self::Int2(TaggedIntType2 {
                0: IntType::Int(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse int value as bytes".to_string()),
                }),
            })),
            _ => Err("Failed to parse value as a ClassIdTypeChoiceCbor".to_string()),
        }
    }
}

/// The `class-id-type-choice` socket is defined in [CoRIM Section 2.1.1].
///
/// ```text
/// $corim-id-type-choice /= tstr
/// $corim-id-type-choice /= uuid-type
/// ```
///
/// [CoRIM Section 2.1.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.1.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CorimIdTypeChoice {
    Str(String),
    Uuid(UuidType),
}
impl TryFrom<&Value> for CorimIdTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Str(s.clone())),
            Value::Bytes(b) => Ok(Self::Uuid(UuidType::Uuid(b.clone()))),
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
impl<'de> serde::Deserialize<'de> for CorimIdTypeChoice {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        let __content = match <serde::__private::de::Content<'_> as serde::Deserialize>::deserialize(
            __deserializer,
        ) {
            serde::__private::Ok(__val) => __val,
            serde::__private::Err(__err) => {
                return serde::__private::Err(__err);
            }
        };
        match &__content {
            Content::String(_s) => {
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <String as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    CorimIdTypeChoice::Str,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            Content::ByteBuf(_b) => {
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <UuidType as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    CorimIdTypeChoice::Uuid,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            _ => {}
        };
        serde::__private::Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum CorimIdTypeChoice",
        ))
    }
}

/// The `comid-role-type-choice` socket is defined in [CoRIM Section 3.1.2].
///
/// ```text
/// $comid-role-type-choice /= &(tag-creator: 0)
/// $comid-role-type-choice /= &(creator: 1)
/// $comid-role-type-choice /= &(maintainer: 2)
/// ```
///
/// [CoRIM Section 3.1.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.2
// restore this derive, the serde(other) line below and the serde-enum-str once no-std support is released
//#[derive(Clone, Debug, Eq, PartialEq, serde_enum_str::Deserialize_enum_str, serde_enum_str::Serialize_enum_str)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum CorimRoleTypeChoice {
    #[serde(rename = "tagCreator")]
    TagCreator,
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "maintainer")]
    Maintainer,
    //#[serde(other)]
    other(String),
}

impl TryFrom<CorimRoleTypeChoice> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: CorimRoleTypeChoice) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoice::TagCreator => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::TagCreator))
            }
            CorimRoleTypeChoice::Creator => Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Creator)),
            CorimRoleTypeChoice::Maintainer => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Maintainer))
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
                CorimRoleTypeChoiceKnownCbor::TagCreator => Ok(Self::TagCreator),
                CorimRoleTypeChoiceKnownCbor::Creator => Ok(Self::Creator),
                CorimRoleTypeChoiceKnownCbor::Maintainer => Ok(Self::Maintainer),
            },
            CorimRoleTypeChoiceCbor::Extensions(e) => Ok(Self::other(e.to_string())),
        }
    }
}

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
            CorimRoleTypeChoice::TagCreator => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::TagCreator))
            }
            CorimRoleTypeChoice::Creator => Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Creator)),
            CorimRoleTypeChoice::Maintainer => {
                Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Maintainer))
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
                CorimRoleTypeChoiceKnownCbor::TagCreator => Ok(Self::TagCreator),
                CorimRoleTypeChoiceKnownCbor::Creator => Ok(Self::Creator),
                CorimRoleTypeChoiceKnownCbor::Maintainer => Ok(Self::Maintainer),
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
                if i.eq(&Integer::from(0)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::TagCreator))
                } else if i.eq(&Integer::from(1)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Creator))
                } else if i.eq(&Integer::from(2)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Maintainer))
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
                if i.eq(&Integer::from(0)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::TagCreator))
                } else if i.eq(&Integer::from(1)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Creator))
                } else if i.eq(&Integer::from(2)) {
                    Ok(Self::Known(CorimRoleTypeChoiceKnownCbor::Maintainer))
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[allow(missing_docs)]
#[repr(i8)]
pub enum CorimRoleTypeChoiceKnownCbor {
    TagCreator = 0,
    Creator = 1,
    Maintainer = 2,
}

/// The `crypto-key-type-choice` socket is defined in [CoRIM Section 3.1.4.1.6].
///
/// ```text
/// $crypto-key-type-choice /= tagged-pkix-base64-key-type
/// $crypto-key-type-choice /= tagged-pkix-base64-cert-type
/// $crypto-key-type-choice /= tagged-pkix-base64-cert-path-type
/// ```
///
/// [CoRIM Section 3.1.4.1.6]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.6
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CryptoKeyTypeChoice {
    Key(TaggedPkixBase64KeyType),
    Cert(TaggedPkixBase64CertType),
    Path(TaggedPkixBase64CertPathType),
}
impl TryFrom<Value> for CryptoKeyTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(554, b) => Ok(Self::Key(TaggedPkixBase64KeyType {
                0: match b.as_text() {
                    Some(t) => t.to_string(),
                    None => {
                        return Err(
                            "Failed to parse CryptoKeyTypeChoice as a text value".to_string()
                        )
                    }
                },
            })),
            Value::Tag(555, b) => Ok(Self::Cert(TaggedPkixBase64CertType {
                0: match b.as_text() {
                    Some(t) => t.to_string(),
                    None => {
                        return Err(
                            "Failed to parse CryptoKeyTypeChoice as a text value".to_string()
                        )
                    }
                },
            })),
            Value::Tag(556, b) => Ok(Self::Path(TaggedPkixBase64CertPathType {
                0: match b.as_text() {
                    Some(t) => t.to_string(),
                    None => {
                        return Err(
                            "Failed to parse CryptoKeyTypeChoice as a text value".to_string()
                        )
                    }
                },
            })),
            _ => Err("Failed to parse value as a CryptoKeyTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for CryptoKeyTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(554, b) => Ok(Self::Key(TaggedPkixBase64KeyType {
                0: match b.as_text() {
                    Some(t) => t.to_string(),
                    None => {
                        return Err(
                            "Failed to parse CryptoKeyTypeChoice as a text value".to_string()
                        )
                    }
                },
            })),
            Value::Tag(555, b) => Ok(Self::Cert(TaggedPkixBase64CertType {
                0: match b.as_text() {
                    Some(t) => t.to_string(),
                    None => {
                        return Err(
                            "Failed to parse CryptoKeyTypeChoice as a text value".to_string()
                        )
                    }
                },
            })),
            Value::Tag(556, b) => Ok(Self::Path(TaggedPkixBase64CertPathType {
                0: match b.as_text() {
                    Some(t) => t.to_string(),
                    None => {
                        return Err(
                            "Failed to parse CryptoKeyTypeChoice as a text value".to_string()
                        )
                    }
                },
            })),
            _ => Err("Failed to parse value as a CryptoKeyTypeChoice".to_string()),
        }
    }
}

/// The `domain-type-choice` socket is defined in [CoRIM Section 3.1.4.1.7].
///
/// ```text
/// $domain-type-choice /= uint
/// $domain-type-choice /= text
/// $domain-type-choice /= tagged-uuid-type
/// ```
///
/// [CoRIM Section 3.1.4.1.7]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.7
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
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err("Failed to parse UUID value as a DomainTypeChoice".to_string())
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
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => return Err("Failed to parse UUID as a DomainTypeChoice".to_string()),
                }),
            })),
            _ => Err("Failed to parse value as a DomainTypeChoice".to_string()),
        }
    }
}

/// The `entity-name-type-choice` socket is defined in [CoRIM Section 1.3.2].
///
/// ```text
/// $entity-name-type-choice /= text
/// ```
///
/// [CoRIM Section 1.3.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-1.3.2
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EntityNameTypeChoice {
    Text(String),
}
impl TryFrom<&Value> for EntityNameTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(EntityNameTypeChoice::Text(s.to_string())),
            _ => Err("Failed to parse value as a EntityNameTypeChoice".to_string()),
        }
    }
}

/// $group-id-type-choice /= tagged-uuid-type
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum GroupIdTypeChoice {
    Uuid(TaggedUuidType),
}
impl TryFrom<Value> for GroupIdTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err("Failed to parse UUID value as a GroupIdTypeChoice".to_string())
                    }
                }),
            })),
            _ => Err("Failed to parse value as a GroupIdTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for GroupIdTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err("Failed to parse UUID value as a GroupIdTypeChoice".to_string())
                    }
                }),
            })),
            _ => Err("Failed to parse value as a GroupIdTypeChoice".to_string()),
        }
    }
}

/// The `instance-id-type-choice` socket is defined in [CoRIM Section 3.1.4.1.3].
///
/// ```text
/// $instance-id-type-choice /= tagged-ueid-type
/// $instance-id-type-choice /= tagged-uuid-type
/// ```
///
/// [CoRIM Section 3.1.4.1.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum InstanceIdTypeChoice {
    Ueid(TaggedUeidType),
    Uuid(TaggedUuidType),
}
impl TryFrom<Value> for InstanceIdTypeChoice {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as a InstanceIdTypeChoice".to_string()
                        )
                    }
                }),
            })),
            Value::Tag(550, b) => Ok(Self::Ueid(TaggedUeidType {
                0: UeidType::Ueid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UEID value as a InstanceIdTypeChoice".to_string()
                        )
                    }
                }),
            })),
            _ => Err("Failed to parse value as a InstanceIdTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for InstanceIdTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as a InstanceIdTypeChoice".to_string()
                        )
                    }
                }),
            })),
            Value::Tag(550, b) => Ok(Self::Ueid(TaggedUeidType {
                0: UeidType::Ueid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UEID value as a InstanceIdTypeChoice".to_string()
                        )
                    }
                }),
            })),
            _ => Err("Failed to parse value as a InstanceIdTypeChoice".to_string()),
        }
    }
}

/// The `measured-element-type-choice` socket is defined in [CoRIM Section 3.1.4.1.5.1].
///
/// ```text
/// $measured-element-type-choice /= tagged-oid-type
/// $measured-element-type-choice /= tagged-uuid-type
/// ```
///
/// [CoRIM Section 3.1.4.1.5.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5.1
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum MeasuredElementTypeChoice {
    Oid(TaggedOidType),
    Uuid(TaggedUuidType),
    Other(Tuple),
}

impl TryFrom<MeasuredElementTypeChoiceCbor> for MeasuredElementTypeChoice {
    type Error = String;
    fn try_from(value: MeasuredElementTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            MeasuredElementTypeChoiceCbor::Oid(b) => Ok(Self::Oid(b.0)),
            MeasuredElementTypeChoiceCbor::Uuid(b) => Ok(Self::Uuid(Required(b.0))),
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
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(OidType::Oid(match b.as_bytes() {
                Some(b) => b.clone(),
                None => {
                    return Err(
                        "Failed to parse OID value as a MeasuredElementTypeChoice".to_string()
                    )
                }
            }))),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as a MeasuredElementTypeChoice".to_string()
                        )
                    }
                }),
            })),
            _ => Err("Failed to parse value as an MeasuredElementTypeChoice".to_string()),
        }
    }
}
impl TryFrom<&Value> for MeasuredElementTypeChoice {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(OidType::Oid(match b.as_bytes() {
                Some(b) => b.clone(),
                None => {
                    return Err(
                        "Failed to parse OID value as a MeasuredElementTypeChoice".to_string()
                    )
                }
            }))),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as a MeasuredElementTypeChoice".to_string()
                        )
                    }
                }),
            })),
            _ => Err(format!(
                "Failed to parse MeasuredElementTypeChoice from value: {:?}",
                value
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum MeasuredElementTypeChoiceCbor {
    Oid(TaggedOidTypeCbor),
    Uuid(TaggedUuidType),
    Other(TupleCbor),
}
impl TryFrom<MeasuredElementTypeChoice> for MeasuredElementTypeChoiceCbor {
    type Error = String;
    fn try_from(value: MeasuredElementTypeChoice) -> Result<Self, Self::Error> {
        match value {
            MeasuredElementTypeChoice::Oid(b) => match b {
                TaggedOidType::Oid(o) => Ok(Self::Oid(Required(OidType::Oid(o)))),
            },
            MeasuredElementTypeChoice::Uuid(b) => Ok(Self::Uuid(Required(b.0))),
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
            MeasuredElementTypeChoice::Oid(b) => match b {
                TaggedOidType::Oid(o) => Ok(Self::Oid(Required(OidType::Oid(o.to_vec())))),
            },
            MeasuredElementTypeChoice::Uuid(b) => Ok(Self::Uuid(Required(b.0.clone()))),
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
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse OID value as an MeasuredElementTypeChoiceCbor"
                                .to_string(),
                        )
                    }
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as an MeasuredElementTypeChoiceCbor"
                                .to_string(),
                        )
                    }
                }),
            })),
            Value::Tag(t, b) => Ok(Self::Other(TupleCbor {
                key: Value::Integer(Integer::from(t)),
                value: *b,
            })),
            _ => Err("Failed to parse value as an MeasuredElementTypeChoiceCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for MeasuredElementTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse OID value as an MeasuredElementTypeChoiceCbor"
                                .to_string(),
                        )
                    }
                }),
            })),
            Value::Tag(37, b) => Ok(Self::Uuid(TaggedUuidType {
                0: UuidType::Uuid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse UUID value as an MeasuredElementTypeChoiceCbor"
                                .to_string(),
                        )
                    }
                }),
            })),
            Value::Tag(t, b) => Ok(Self::Other(TupleCbor {
                key: Value::Integer(Integer::from(*t)),
                value: *b.clone(),
            })),
            _ => Err("Failed to parse value as an MeasuredElementTypeChoiceCbor".to_string()),
        }
    }
}

/// The `profile-type-choice` socket is defined in [CoRIM Section 2.1.4].
///
/// ```text
/// profile-type-choice = uri / tagged-oid-type
/// ```
///
/// [CoRIM Section 2.1.4]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.1.4
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
            Value::Bytes(s) => Ok(Self::Oid2(OidType::Oid(s))),
            Value::Tag(111, b) => Ok(Self::Oid(OidType::Oid(match b.as_bytes() {
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
            Value::Bytes(s) => Ok(Self::Oid2(OidType::Oid(s.clone()))),
            Value::Tag(111, b) => Ok(Self::Oid(OidType::Oid(match b.as_bytes() {
                Some(b) => b.clone(),
                None => return Err("Failed to parse OID value as an ProfileTypeChoice".to_string()),
            }))),
            _ => Err("Failed to parse value as a ProfileTypeChoice".to_string()),
        }
    }
}

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
            ProfileTypeChoice::Oid(b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b {
                    TaggedOidType::Oid(b) => b,
                }),
            })),
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
            ProfileTypeChoice::Oid(b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b {
                    TaggedOidType::Oid(b) => b.clone(),
                }),
            })),
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
            Value::Bytes(s) => Ok(Self::Oid2(OidType::Oid(s))),
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse OID value as an ProfileTypeChoiceCbor".to_string()
                        )
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
            Value::Bytes(s) => Ok(Self::Oid2(OidType::Oid(s.clone()))),
            Value::Tag(111, b) => Ok(Self::Oid(TaggedOidTypeCbor {
                0: OidType::Oid(match b.as_bytes() {
                    Some(b) => b.clone(),
                    None => {
                        return Err(
                            "Failed to parse OID value as an ProfileTypeChoiceCbor".to_string()
                        )
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

/// The `svn-type-choice` socket is defined in [CoRIM Section 3.1.4.1.5.4].
///
/// ```text
/// svn-type-choice = tagged-svn / tagged-min-svn
/// ```
///
/// [CoRIM Section 3.1.4.1.5.4]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5.4
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
                        )
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
                        )
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
                        )
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
                        )
                    }
                },
            })),
            _ => Err("Failed to parse value as an SvnTypeChoice".to_string()),
        }
    }
}

/// The `tag-id-type-choice` socket is defined in [CoRIM Section 3.1.1.1].
///
/// ```text
/// $tag-id-type-choice /= tstr
/// $tag-id-type-choice /= uuid-type
/// ```
///
/// [CoRIM Section 3.1.1.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.1.1
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
impl TryFrom<&Value> for TagIdTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(s) => Ok(Self::Str(s.clone())),
            Value::Bytes(b) => Ok(Self::Uuid(UuidType::Uuid(b.clone()))),
            _ => Err("Failed to parse value as a TagIdTypeChoiceCbor".to_string()),
        }
    }
}
// Serde does not parse untagged enums properly (it just parses as the first type)
impl<'de> serde::Deserialize<'de> for TagIdTypeChoiceCbor {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        let __content = match <serde::__private::de::Content<'_> as serde::Deserialize>::deserialize(
            __deserializer,
        ) {
            serde::__private::Ok(__val) => __val,
            serde::__private::Err(__err) => {
                return serde::__private::Err(__err);
            }
        };
        match &__content {
            Content::String(_s) => {
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <String as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    TagIdTypeChoiceCbor::Str,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            Content::ByteBuf(_b) => {
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <UuidType as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    TagIdTypeChoiceCbor::Uuid,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            _ => {}
        };
        serde::__private::Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum TagIdTypeChoiceCbor",
        ))
    }
}

/// The `tag-rel-type-choice` socket is defined in [CoRIM Section 3.1.3].
///
/// ```text
/// $tag-rel-type-choice /= &(supplements: 0)
/// $tag-rel-type-choice /= &(replaces: 1)
/// ```
///
/// [CoRIM Section 3.1.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.3
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagRelTypeChoice {
    Known(TagRelTypeChoiceKnown),
    Extensions(i8),
}

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
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
/// The `tag-version-type` socket is defined in [CoRIM Section 3.1.1.2].
///
/// ```text
/// tag-version-type = uint .default 0
/// ```
///
/// [CoRIM Section 3.1.1.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.1.2
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
