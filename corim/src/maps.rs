//! Map-based structs from the Concise Reference Integrity Manifest (CoRIM) spec

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use ciborium::value::Integer;
use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    __private::size_hint,
    de::{Error, MapAccess, Visitor},
};

use crate::arrays::*;
use crate::choices::*;
use cbor_derive::StructToMap;
use common::arrays::*;
use common::choices::*;
use common::*;
use serde::ser::Error as OtherError;

/// The `class-map` type is defined in [CoRIM Section 3.1.4.1.2].
///
/// ```text
/// class-map = non-empty<{
///   ? &(class-id: 0) => $class-id-type-choice
///   ? &(vendor: 1) => tstr
///   ? &(model: 2) => tstr
///   ? &(layer: 3) => uint
///   ? &(index: 4) => uint
/// }>
/// ```
///
/// [CoRIM Section 3.1.4.1.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ClassMap {
    #[cbor(tag = "0", cbor = "true")]
    pub id: Option<ClassIdTypeChoice>,
    #[cbor(tag = "1", value = "Text")]
    pub vendor: Option<String>,
    #[cbor(tag = "2", value = "Text")]
    pub model: Option<String>,
    #[cbor(tag = "3", value = "Integer")]
    pub layer: Option<u128>,
    #[cbor(tag = "4", value = "Integer")]
    pub index: Option<u128>,
}

/// The `concise-mid-tag` type is defined in [CoRIM Section 3.1].
///
/// ```text
/// concise-mid-tag = {
///   ? &(language: 0) => text
///   &(tag-identity: 1) => tag-identity-map
///   ? &(entities: 2) => [ + entity-map ]
///   ? &(linked-tags: 3) => [ + linked-tag-map ]
///   &(triples: 4) => triples-map
///   * $$concise-mid-tag-extension
/// }
/// ```
///
/// [CoRIM Section 3.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConciseMidTag {
    #[cbor(tag = "0", value = "Text")]
    pub language: Option<String>,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    #[serde(rename = "tag-identity")]
    pub tag_identity: Option<TagIdentityMap>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub entities: Option<Vec<EntityMap>>,
    #[cbor(tag = "3", value = "Array", cbor = "true")]
    pub linked_tags: Option<Vec<LinkedTagMap>>,
    #[cbor(tag = "4", value = "Map", cbor = "true")]
    pub triples: TriplesMap,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `corim-locator-map` type is defined in [CoRIM Section 2.1.3].
///
/// ```text
/// corim-locator-map = {
///   &(href: 0) => uri
///   ? &(thumbprint: 1) => hash-entry
/// }
/// ```
///
/// [CoRIM Section 2.1.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.1.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CorimLocatorMap {
    //todo is taggeduri right?
    #[cbor(tag = "0", cbor = "true")]
    pub href: TaggedUriType,
    #[cbor(tag = "1", cbor = "true")]
    pub thumbprint: Option<HashEntry>,
}

/// The `corim-map` type is defined in [CoRIM Section 2.1].
///
/// ```text
/// corim-map = {
///   &(id: 0) => $corim-id-type-choice
///   &(tags: 1) => [ + $concise-tag-type-choice ]
///   ? &(dependent-rims: 2) => [ + corim-locator-map ]
///   ? &(profile: 3) => [ + profile-type-choice ]
///   ? &(rim-validity: 4) => validity-map
///   ? &(entities: 5) => [ + corim-entity-map ]
///   * $$corim-map-extension
/// }
/// ```
///
/// [CoRIM Section 2.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CorimMap {
    #[cbor(tag = "0")]
    pub id: CorimIdTypeChoice,
    #[cbor(tag = "1", value = "Array")]
    pub tags: Vec<ConciseTagTypeChoice>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub dependent_rims: Option<Vec<CorimLocatorMap>>,
    #[cbor(tag = "3", value = "Array", cbor = "true")]
    pub profile: Option<Vec<ProfileTypeChoice>>,
    #[cbor(tag = "4", value = "Map", cbor = "true")]
    pub rim_validity: Option<ValidityMap>,
    #[cbor(tag = "5", value = "Array", cbor = "true")]
    pub entities: Option<Vec<EntityMap>>,
    //todo extensibility
    //extensions
}

/// The `coswid-triple-record` type is defined in [CoRIM Section 2.2.2].
///
/// ```text
/// corim-meta-map = {
///   &(signer: 0) => corim-signer-map
///   ? &(signature-validity: 1) => validity-map
/// }
/// ```
///
/// [CoRIM Section 2.2.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.2.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CorimMetaMap {
    #[cbor(tag = "0", value = "Map", cbor = "true")]
    pub signer: CorimSignerMap,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub validity: Option<ValidityMap>,
}

/// The `corim-signer-map` type is defined in [CoRIM Section 2.2.2.1].
///
/// ```text
/// corim-signer-map = {
///   &(signer-name: 0) => $entity-name-type-choice
///   ? &(signer-uri: 1) => uri
///   * $$corim-signer-map-extension
/// }
/// ```
///
/// [CoRIM Section 2.2.2.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.2.2.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CorimSignerMap {
    #[cbor(tag = "0")]
    pub entity_name: EntityNameTypeChoice,
    #[cbor(tag = "1", cbor = "true")]
    pub reg_id: Option<TaggedUriType>,
    //todo extensibility
    //extensions
}

/// The `entity-map` type is defined in [CoRIM Section 1.3.2].
///
/// ```text
/// entity-map<role-type-choice, extension-socket> = {
///   &(entity-name: 0) => $entity-name-type-choice
///   ? &(reg-id: 1) => uri
///   &(role: 2) => [ + role-type-choice ]
///   * extension-socket
/// }
/// ```
///
/// [CoRIM Section 1.3.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-1.3.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EntityMap {
    #[cbor(tag = "0")]
    pub name: EntityNameTypeChoice,
    #[cbor(tag = "1", cbor = "true")]
    pub regid: Option<TaggedUriType>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub roles: Vec<CorimRoleTypeChoice>,
    //todo extensibility
    //extensions
}

/// The `environment-map` type is defined in [CoRIM Section 3.1.4.1.1].
///
/// ```text
/// environment-map = non-empty<{
///   ? &(class: 0) => class-map
///   ? &(instance: 1) => $instance-id-type-choice
///   ? &(group: 2) => $group-id-type-choice
/// }>
/// ```
///
/// [CoRIM Section 3.1.4.1.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentMap {
    #[cbor(tag = "0", cbor = "true")]
    pub class: Option<ClassMap>,
    #[cbor(tag = "1")]
    pub instance: Option<InstanceIdTypeChoice>,
    #[cbor(tag = "2")]
    pub group: Option<GroupIdTypeChoice>,
    //todo extensibility
    //extensions
}

/// The `flags-map` type is defined in [CoRIM Section 3.1.4.1.5.5].
///
/// ```text
///   flags-map = {
///      ? &(configured: 0) => bool
///      ? &(secure: 1) => bool
///      ? &(recovery: 2) => bool
///      ? &(debug: 3) => bool
///      ? &(replay-protected: 4) => bool
///      ? &(integrity-protected: 5) => bool
///      * $$flags-map-extension
///    }
/// ```
///
/// [CoRIM Section 3.1.4.1.5.5]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5.5
// #[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
// #[allow(missing_docs)]
// pub struct FlagsMap {
//     #[cbor(tag = "0", value = "Bool")]
//     pub configured: Option<bool>,
//     #[cbor(tag = "1", value = "Bool")]
//     pub secure: Option<bool>,
//     #[cbor(tag = "2", value = "Bool")]
//     pub recovery: Option<bool>,
//     #[cbor(tag = "3", value = "Bool")]
//     pub debug: Option<bool>,
//     #[cbor(tag = "4", value = "Bool")]
//     pub replay_protected: Option<bool>,
//     #[cbor(tag = "5", value = "Bool")]
//     pub integrity_protected: Option<bool>,
//     #[cbor(value = "Array", cbor = "true")]
//     pub other: Option<Vec<Tuple>>,
// }
//todo using i8 to align with comid repo output (it is disaligned with corim spec)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlagsMap(i8);
impl TryFrom<&Value> for FlagsMap {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => {
                let v: i8 = match Integer::try_into(*i) {
                    Ok(i) => i,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FlagsMap(v))
            }
            _ => Err("Failed to parse value as a FlagsMap".to_string()),
        }
    }
}

/// The `linked-tag-map` type is defined in [CoRIM Section 3.1.3].
///
/// ```text
/// linked-tag-map = {
///   &(linked-tag-id: 0) => $tag-id-type-choice
///   &(tag-rel: 1) => $tag-rel-type-choice
/// }
/// ```
///
/// [CoRIM Section 3.1.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LinkedTagMap {
    #[cbor(tag = "0", cbor = "true")]
    pub linked_tag_id: TagIdTypeChoice,
    #[cbor(tag = "1")]
    pub tag_rel: TagRelTypeChoice,
}

/// The `measurement-map` type is defined in [CoRIM Section 3.1.4.1.5].
///
/// ```text
/// measurement-map = {
///   ? &(mkey: 0) => $measured-element-type-choice
///   &(mval: 1) => measurement-values-map
/// }
/// ```
///
/// [CoRIM Section 3.1.4.1.5]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementMap {
    #[cbor(tag = "0", cbor = "true")]
    pub mkey: Option<MeasuredElementTypeChoice>,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub value: MeasurementValuesMap,
}

/// The `measurement-values-map` type is defined in [CoRIM Section 3.1.4.1.5.2].
///
/// ```text
/// measurement-values-map = non-empty<{
///   ? &(version: 0) => version-map
///   ? &(svn: 1) => svn-type-choice
///   ? &(digests: 2) => [ + hash-entry ]
///   ? &(flags: 3) => flags-map
///   ? (
///       &(raw-value: 4) => $raw-value-type-choice,
///       ? &(raw-value-mask: 5) => raw-value-mask-type
///     )
///   ? &(mac-addr: 6) => mac-addr-type-choice
///   ? &(ip-addr: 7) =>  ip-addr-type-choice
///   ? &(serial-number: 8) => text
///   ? &(ueid: 9) => ueid-type
///   ? &(uuid: 10) => uuid-type
///   ? &(name: 11) => text
///   * $$measurement-values-map-extension
/// }>
/// ```
///
/// [CoRIM Section 3.1.4.1.5.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementValuesMap {
    #[cbor(tag = "0", value = "Map", cbor = "true")]
    pub version: Option<VersionMap>,
    #[cbor(tag = "1")]
    pub svn: Option<SvnTypeChoice>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub digests: Option<Vec<HashEntry>>,
    #[cbor(tag = "3")]
    #[serde(rename = "op-flags")]
    pub flags: Option<FlagsMap>,
    //todo raw field support
    // #[cbor(tag = "4")]
    // pub raw_value: Option<RawValueTypeChoice>,
    // #[cbor(tag = "5")]
    // pub raw_value_mask: Option<RawValueMaskType>,
    #[cbor(tag = "6", value = "Bytes")]
    pub mac_addr: Option<Vec<u8>>,
    #[cbor(tag = "7", value = "Bytes")]
    pub ip_addr: Option<Vec<u8>>,
    #[cbor(tag = "8", value = "Text")]
    pub serial_number: Option<String>,
    #[cbor(tag = "9")]
    pub ueid: Option<UeidType>,
    #[cbor(tag = "10")]
    pub uuid: Option<UuidType>,
    #[cbor(tag = "11", value = "Text")]
    pub name: Option<String>,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `protected-corim-header-map` type is defined in [CoRIM Section 2.2.1].
///
/// ```text
/// protected-corim-header-map = {
///   &(alg-id: 1) => int
///   &(content-type: 3) => "application/corim-unsigned+cbor"
///   &(issuer-key-id: 4) => bstr
///   &(corim-meta: 8) => bstr .cbor corim-meta-map
///   * cose-label => cose-value
/// }
/// ```
///
/// [CoRIM Section 2.2.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-2.2.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ProtectedCorimHeaderMap {
    #[cbor(tag = "0", value = "Integer")]
    pub alg_id: u64,
    #[cbor(tag = "3", value = "Text")]
    content_type: String,
    #[cbor(tag = "4", value = "Bytes")]
    pub issuer_key_id: Vec<u8>,
    #[cbor(tag = "8", value = "Map", cbor = "true")]
    pub meta: CorimMetaMap,
    // todo CoseValues
    //pub cose_label: CoseValues
}

/// The `coswid-triple-record` type is defined in [CoRIM Section 3.1.1].
///
/// ```text
/// tag-identity-map = {
///   &(tag-id: 0) => $tag-id-type-choice
///   ? &(tag-version: 1) => tag-version-type
/// }
/// ```
///
/// [CoRIM Section 3.1.1]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TagIdentityMap {
    #[cbor(tag = "0", cbor = "true")]
    #[serde(rename = "id")]
    pub tag_id: TagIdTypeChoice,
    //todo defaults to zero
    #[cbor(tag = "1")]
    #[serde(rename = "version")]
    pub tag_version: Option<TagVersionType>,
}

// todo non-empty
/// The `triples-map` type is defined in [CoRIM Section 3.1.4].
///
/// ```text
/// triples-map = non-empty<{
///   ? &(reference-triples: 0) => [ + reference-triple-record ]
///   ? &(endorsed-triples: 1)  => [ + endorsed-triple-record ]
///   ? &(identity-triples: 2) => [ + identity-triple-record ]
///   ? &(attest-key-triples: 3) => [ + attest-key-triple-record ]
///   ? &(dependency-triples: 4) => [ + domain-dependency-triple-record ]
///   ? &(membership-triples: 5) => [ + domain-membership-triple-record ]
///   ? &(coswid-triples: 6) => [ + coswid-triple-record ]
///   * $$triples-map-extension
/// }>
/// ```
///
/// [CoRIM Section 3.1.4]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct TriplesMap {
    #[cbor(tag = "0", value = "Array", cbor = "true")]
    #[serde(rename = "reference-values")]
    pub reference_triples: Option<Vec<ReferenceTripleRecord>>,
    #[cbor(tag = "1", value = "Array", cbor = "true")]
    pub endorsed_triples: Option<Vec<EndorsedTripleRecord>>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub identity_triples: Option<Vec<IdentityTripleRecord>>,
    #[cbor(tag = "3", value = "Array", cbor = "true")]
    pub attest_key_triples: Option<Vec<AttestKeyTripleRecord>>,
    #[cbor(tag = "4", value = "Array", cbor = "true")]
    pub dependency_triples: Option<Vec<DomainDependencyTripleRecord>>,
    #[cbor(tag = "5", value = "Array", cbor = "true")]
    pub membership_triples: Option<Vec<DomainDependencyTripleRecord>>,
    #[cbor(tag = "6", value = "Array", cbor = "true")]
    pub coswid_triples: Option<Vec<CoswidTripleRecord>>,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `validity-map` type is defined in [CoRIM Section 1.3.3].
///
/// ```text
/// validity-map = {
///   ? &(not-before: 0) => time
///   &(not-after: 1) => time
/// }
/// ```
///
/// [CoRIM Section 1.3.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-1.3.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ValidityMap {
    #[cbor(tag = "0", cbor = "true")]
    pub not_before: Option<Time>,
    #[cbor(tag = "1", cbor = "true")]
    pub not_after: Time,
}

// todo - the corim sample uses this struct still
/// The `verification-key-map` type is not defined in the current CoRIM but is used
/// in samples generated by the reference implementation (it had been in -02).
///
/// ```text
///    verification-key-map = {
///      comid.key => pkix-base64-key-type
///      ? comid.keychain => [ + pkix-base64-cert-type ]
///    }
/// ```
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct VerificationKeyMap {
    #[cbor(tag = "0", value = "Text")]
    pub key: String,
    #[cbor(tag = "1", value = "Array")]
    pub keychain: Option<Vec<PkixBase64Type>>,
}

/// The `version-map` type is defined in [CoRIM Section 3.1.4.1.5.3].
///
/// ```text
/// version-type = text .default '0.0.0'
/// version-map = {
///   &(version: 0) => text
///   ? &(version-scheme: 1) => $version-scheme
/// }
/// ```
///
/// [CoRIM Section 3.1.4.1.5.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.1.5.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct VersionMap {
    #[cbor(tag = "0", value = "Text")]
    pub version: String,
    #[cbor(tag = "1")]
    pub version_scheme: Option<VersionScheme>,
}
