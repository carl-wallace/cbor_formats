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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[cbor(companion = "true")]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "value")]
pub enum ClassIdTypeChoice {
    #[cbor(tag = "111", cbor = "true")]
    oid(OidType),
    #[cbor(tag = "37", cbor = "true")]
    uuid(UuidType),
    #[cbor(tag = "560", cbor = "true")]
    bytes(BytesType),
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
/// JSON representation of `$corim-role-type-choice`.
///
/// ```text
/// $corim-role-type-choice /= &(tag-creator: 0)
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
    EnumToChoice,
)]
#[cbor(companion = "true")]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum CorimRoleTypeChoice {
    #[serde(rename = "tagCreator")]
    #[cbor(tag = "0")]
    TagCreator,
    #[serde(rename = "manifestCreator")]
    #[cbor(tag = "1")]
    ManifestCreator,
    #[serde(rename = "manifestSigner")]
    #[cbor(tag = "2")]
    ManifestSigner,
    #[serde(other)]
    other(String),
}

/// JSON representation of `$comid-role-type-choice`.
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
    EnumToChoice,
)]
#[cbor(companion = "true")]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum ComidRoleTypeChoice {
    #[serde(rename = "tagCreator")]
    #[cbor(tag = "0")]
    TagCreator,
    #[serde(rename = "creator")]
    #[cbor(tag = "1")]
    Creator,
    #[serde(rename = "maintainer")]
    #[cbor(tag = "2")]
    Maintainer,
    #[serde(other)]
    other(String),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[cbor(companion = "true")]
#[serde(tag = "type", content = "value")]
#[allow(missing_docs)]
pub enum MeasuredElementTypeChoice {
    #[cbor(tag = "111", cbor = "true")]
    Oid(OidType),
    #[cbor(tag = "37", cbor = "true")]
    Uuid(UuidType),
    #[cbor(value = "Integer")]
    Uint(u64),
    #[cbor(value = "Text")]
    Text(String),
    #[cbor(socket = "true")]
    Other(Tuple),
}

/// The `profile-type-choice` socket is defined in [CoRIM Section 4.1.4].
///
/// ```text
/// profile-type-choice = uri / tagged-oid-type
/// ```
///
/// [CoRIM Section 4.1.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1.4
/// CoRIM profile: `uri / tagged-oid-type` per [CoRIM Section 4.1.4].
///
/// For EAT/EAR profiles (`general-uri / general-oid` with bare untagged OID),
/// use [`common::GeneralProfile`] instead.
///
/// [CoRIM Section 4.1.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1.4
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumToChoice)]
#[cbor(companion = "true")]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ProfileTypeChoice {
    #[cbor(value = "Text")]
    Uri(Uri),
    #[cbor(tag = "111", cbor = "true")]
    Oid(OidType),
    #[cbor(socket = "true")]
    Other(Tuple),
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
