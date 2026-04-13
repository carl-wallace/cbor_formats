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
use cbor_derive::EnumToChoice;
use ciborium::value::Value;
use common::*;
use core::fmt;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

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

/// The `$concise-tag-type-choice` socket.
///
/// ```text
/// $concise-tag-type-choice /= #6.505(bytes .cbor concise-swid-tag)
/// $concise-tag-type-choice /= #6.506(bytes .cbor concise-mid-tag)
/// ```
///
/// Represented as opaque bytes rather than a typed enum. Tags are embedded as
/// `bstr` blobs in the CoRIM `tags` array, and parsing them eagerly would
/// couple the CoRIM parser to the full CoSWID and CoMID schemas. Callers can
/// decode individual tags on demand using the appropriate crate.
pub type ConciseTagTypeChoice = BytesType;

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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ClassIdTypeChoiceCbor {
    #[cbor(tag = "111", cbor = "true")]
    Oid(TaggedOidTypeCbor),
    #[cbor(tag = "37", cbor = "true")]
    Uuid(TaggedUuidType),
    #[cbor(tag = "560", cbor = "true")]
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

/// The `corim-id-type-choice` socket is defined in [CoRIM Section 4.1.1].
///
/// ```text
/// $corim-id-type-choice /= tstr
/// $corim-id-type-choice /= uuid-type
/// ```
///
/// [CoRIM Section 4.1.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1.1
#[derive(Clone, Debug, Eq, PartialEq, Serialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CorimIdTypeChoice {
    #[cbor(value = "Text")]
    Str(String),
    #[cbor(value = "Bytes")]
    Uuid(UuidType),
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
///
/// Also includes `$comid-role-type-choice` variants because [`EntityMap`]
/// is shared between CoRIM and CoMID entity maps:
///
/// ```text
/// $comid-role-type-choice /= &(tag-creator: 0)
/// $comid-role-type-choice /= &(creator: 1)
/// $comid-role-type-choice /= &(maintainer: 2)
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
    #[serde(rename = "tagCreator")]
    TagCreator,
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "maintainer")]
    Maintainer,
    #[serde(other)]
    other(String),
}

impl TryFrom<CorimRoleTypeChoice> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: CorimRoleTypeChoice) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<CorimRoleTypeChoiceCbor> for CorimRoleTypeChoice {
    type Error = String;
    fn try_from(value: CorimRoleTypeChoiceCbor) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// CBOR-encodable counterpart of [`CorimRoleTypeChoice`].
///
/// Carries the raw integer value from the wire. Known values are available as
/// associated constants for comparison. The `$` socket in CDDL means any
/// integer is valid — unknown values are extensions, not errors.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CorimRoleTypeChoiceCbor {
    #[cbor(value = "Integer")]
    Value(i8),
}

impl CorimRoleTypeChoiceCbor {
    /// `&(tag-creator: 0)` — both `$corim-role-type-choice` and `$comid-role-type-choice`
    pub const TAG_CREATOR: i8 = 0;
    /// `&(manifest-creator: 1)` / `&(creator: 1)`
    pub const MANIFEST_CREATOR: i8 = 1;
    /// `&(manifest-signer: 2)` / `&(maintainer: 2)`
    pub const MANIFEST_SIGNER: i8 = 2;
}

impl TryFrom<&CorimRoleTypeChoice> for CorimRoleTypeChoiceCbor {
    type Error = String;
    fn try_from(value: &CorimRoleTypeChoice) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoice::TagCreator => Ok(Self::Value(Self::TAG_CREATOR)),
            CorimRoleTypeChoice::ManifestCreator | CorimRoleTypeChoice::Creator => {
                Ok(Self::Value(Self::MANIFEST_CREATOR))
            }
            CorimRoleTypeChoice::ManifestSigner | CorimRoleTypeChoice::Maintainer => {
                Ok(Self::Value(Self::MANIFEST_SIGNER))
            }
            CorimRoleTypeChoice::other(s) => {
                Ok(Self::Value(s.parse::<i8>().map_err(|e| e.to_string())?))
            }
        }
    }
}

impl TryFrom<&CorimRoleTypeChoiceCbor> for CorimRoleTypeChoice {
    type Error = String;
    fn try_from(value: &CorimRoleTypeChoiceCbor) -> Result<Self, Self::Error> {
        match value {
            CorimRoleTypeChoiceCbor::Value(v) => match *v {
                CorimRoleTypeChoiceCbor::TAG_CREATOR => Ok(Self::TagCreator),
                CorimRoleTypeChoiceCbor::MANIFEST_CREATOR => Ok(Self::ManifestCreator),
                CorimRoleTypeChoiceCbor::MANIFEST_SIGNER => Ok(Self::ManifestSigner),
                other => Ok(Self::other(other.to_string())),
            },
        }
    }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum CryptoKeyTypeChoice {
    #[cbor(tag = "554", cbor = "true")]
    Key(TaggedPkixBase64KeyType),
    #[cbor(tag = "555", cbor = "true")]
    Cert(TaggedPkixBase64CertType),
    #[cbor(tag = "556", cbor = "true")]
    Path(TaggedPkixBase64CertPathType),
    #[cbor(tag = "557", cbor = "true")]
    KeyThumbprint(TaggedKeyThumbprintType),
    #[cbor(tag = "558", cbor = "true")]
    CoseKey(TaggedCoseKeyType),
    #[cbor(tag = "559", cbor = "true")]
    CertThumbprint(TaggedCertThumbprintType),
    #[cbor(tag = "560", cbor = "true")]
    Bytes(TaggedBytes),
    #[cbor(tag = "561", cbor = "true")]
    CertPathThumbprint(TaggedCertPathThumbprintType),
    #[cbor(tag = "562", cbor = "true")]
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

/// The `domain-type-choice` socket is defined in [CoRIM Section 5.1.11].
///
/// ```text
/// $domain-type-choice /= uint
/// $domain-type-choice /= text
/// $domain-type-choice /= tagged-uuid-type
/// ```
///
/// [CoRIM Section 5.1.11]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.11
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum DomainTypeChoice {
    #[cbor(value = "Integer")]
    U64(u64),
    #[cbor(value = "Text")]
    Text(String),
    #[cbor(tag = "37", cbor = "true")]
    Uuid(TaggedUuidType),
}

/// The `entity-name-type-choice` socket is defined in [CoRIM Section 7.2].
///
/// [CoRIM Section 7.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.2
///
/// ```text
/// $entity-name-type-choice /= text
/// $entity-name-type-choice /= tagged-oid-type
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EntityNameTypeChoice {
    #[cbor(value = "Text")]
    Text(String),
    #[cbor(tag = "111", cbor = "true")]
    Oid(TaggedOidTypeCbor),
}

/// $group-id-type-choice /= tagged-uuid-type / tagged-bytes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum GroupIdTypeChoice {
    #[cbor(tag = "37", cbor = "true")]
    Uuid(TaggedUuidType),
    #[cbor(tag = "560", cbor = "true")]
    Bytes(TaggedBytes),
}

/// The `instance-id-type-choice` socket is defined in [CoRIM Section 5.1.4.3].
///
/// [CoRIM Section 5.1.4.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.3
///
/// Includes tagged-ueid-type, tagged-uuid-type, and all crypto-key-type-choice variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum InstanceIdTypeChoice {
    #[cbor(tag = "550", cbor = "true")]
    Ueid(TaggedUeidType),
    #[cbor(tag = "37", cbor = "true")]
    Uuid(TaggedUuidType),
    CryptoKey(CryptoKeyTypeChoice),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum MeasuredElementTypeChoiceCbor {
    #[cbor(tag = "111", cbor = "true")]
    Oid(TaggedOidTypeCbor),
    #[cbor(tag = "37", cbor = "true")]
    Uuid(TaggedUuidType),
    #[cbor(value = "Integer")]
    Uint(u64),
    #[cbor(value = "Text")]
    Text(String),
    #[cbor(socket = "true")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ProfileTypeChoiceCbor {
    #[cbor(value = "Text")]
    Uri(Uri),
    #[cbor(tag = "111", cbor = "true")]
    Oid(TaggedOidTypeCbor),
    #[cbor(value = "Bytes")]
    Oid2(OidType),
    #[cbor(socket = "true")]
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

/// The `svn-type-choice` socket is defined in [CoRIM Section 5.1.4.5.4].
///
/// ```text
/// svn-type-choice = tagged-svn / tagged-min-svn
/// ```
///
/// [CoRIM Section 5.1.4.5.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.4
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SvnTypeChoice {
    #[cbor(tag = "552", cbor = "true")]
    TaggedSvn(TaggedSvn),
    #[cbor(tag = "553", cbor = "true")]
    TaggedMinSvn(TaggedMinSvn),
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagIdTypeChoiceCbor {
    #[cbor(value = "Text")]
    Str(String),
    #[cbor(value = "Bytes")]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagRelTypeChoice {
    #[cbor(value = "Integer")]
    Value(i8),
}

impl TagRelTypeChoice {
    /// `&(supplements: 0)`
    pub const SUPPLEMENTS: i8 = 0;
    /// `&(replaces: 1)`
    pub const REPLACES: i8 = 1;
}

/// The `tag-version-type` socket is defined in [CoRIM Section 5.1.1.2].
///
/// ```text
/// tag-version-type = uint .default 0
/// ```
///
/// The CDDL `.default 0` means the field may be omitted on the wire;
/// when absent, the semantic value is 0.
///
/// [CoRIM Section 5.1.1.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.1.2
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TagVersionType {
    #[cbor(value = "Integer")]
    U64(u64),
}

impl Default for TagVersionType {
    fn default() -> Self {
        Self::U64(0)
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum RawValueTypeChoice {
    #[cbor(tag = "560", cbor = "true")]
    Bytes(TaggedBytes),
    #[cbor(tag = "563", cbor = "true")]
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

/// The `int-range-type-choice` is defined in [CoRIM Section 5.1.4.8].
///
/// [CoRIM Section 5.1.4.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.8
///
/// ```text
/// $int-range-type-choice /= int
/// $int-range-type-choice /= tagged-int-range ; #6.564
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum IntRangeTypeChoice {
    #[cbor(value = "Integer")]
    Int(i64),
    #[cbor(tag = "564", cbor = "true")]
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
