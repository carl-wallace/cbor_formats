//! Map-based structs from the Concise Trust Anchor Store (CoTS) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    __private::size_hint,
    de::{Error, MapAccess, Visitor},
};

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

use crate::arrays::*;
use crate::choices::TasListPurpose;
use cbor_derive::StructToMap;
use common::choices::*;
use common::*;
use corim::choices::TagVersionType;
use corim::maps::*;
use coswid::maps::*;
use eat::maps::ClaimsSetClaims;
use eat::maps::*;
use serde::ser::Error as OtherError;

/// abbreviated-swid-tag = {
///   ? tag-version => integer,
///   ? corpus => bool,
///   ? patch => bool,
///   ? supplemental => bool,
///   ? software-name => text,
///   ? software-version => text,
///   ? version-scheme => $version-scheme,
///   ? media => text,
///   ? software-meta => one-or-more<software-meta-entry>,
///   ? entity => one-or-more<entity-entry>,
///   ? link => one-or-more<link-entry>,
///   ? payload-or-evidence,
///   * $$coswid-extension,
///   global-attributes,
/// }
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AbbreviatedSwidTag {
    #[cbor(tag = "12")]
    pub tag_version: Option<TagVersionType>,
    #[cbor(tag = "8", value = "Bool")]
    pub corpus: Option<bool>,
    #[cbor(tag = "9", value = "Bool")]
    pub patch: Option<bool>,
    #[cbor(tag = "11", value = "Bool")]
    pub supplemental: Option<bool>,
    #[cbor(tag = "1", value = "Text")]
    pub software_name: Option<String>,
    #[cbor(tag = "13", value = "Text")]
    pub software_version: Option<String>,
    #[cbor(tag = "13")]
    pub version_scheme: Option<VersionScheme>,
    #[cbor(tag = "10", value = "Text")]
    pub media: Option<String>,
    #[cbor(tag = "5", cbor = "true")]
    pub software_meta: Option<OneOrMoreSoftwareMetaEntry>,
    #[cbor(tag = "2", cbor = "true")]
    pub entity: Option<OneOrMoreEntityEntry>,
    #[cbor(tag = "4", cbor = "true")]
    pub link: Option<OneOrMoreLinkEntry>,
    #[cbor(tag = "3", cbor = "true")]
    pub evidence: Option<EvidenceEntry>,
    #[cbor(tag = "6", cbor = "true")]
    pub payload: Option<PayloadEntry>,
    //* $$coswid-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //global-attributes,
    // todo extensions and attributes
}

// ; cas-and-tas-map indices
// tastore.tas = 0
// tastore.cas = 1
// cas-and-tas-map = {
//  tastore.tas => [ + trust-anchor ]
//  ? tastore.cas => [ + tastore.pkix-cert-type ]
// }
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CasAndTasMap {
    #[cbor(tag = "0", value = "Array", cbor = "true")]
    pub tas: Vec<TrustAnchor>,
    #[cbor(tag = "1", value = "Array")]
    pub cas: Option<Vec<PkixCa>>,
}

// ; concise-ta-store-map indices
// tastore.language = 0
// tastore.store-identity = 1
// tastore.environment = 2
// tastore.purpose = 3
// tastore.perm_claims = 4
// tastore.excl_claims = 5
// tastore.keys = 6
// concise-ta-store-map = {
//  ? tastore.language => language-type
//  ? tastore.store-identity => tag-identity-map
//  tastore.environments => environment-group-list
//  ? tastore.purposes => [+ $$tas-list-purpose]
//  ? tastore.perm_claims => [+ $$claims-set-claims]
//  ? tastore.excl_claims => [+ $$claims-set-claims]
//  tastore.keys => cas-and-tas-map
// }
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConciseTaStoreMap {
    #[cbor(tag = "0", value = "Text")]
    pub language: Option<String>,
    #[cbor(tag = "1", cbor = "true")]
    pub store_identity: Option<TagIdentityMap>,
    #[cbor(tag = "2", cbor = "true")]
    pub environments: EnvironmentGroupList,
    #[cbor(tag = "3", value = "Array")]
    pub purposes: Option<Vec<TasListPurpose>>,
    #[cbor(tag = "4", value = "Array", cbor = "true")]
    pub perm_claims: Option<Vec<ClaimsSetClaims>>,
    #[cbor(tag = "5", value = "Array", cbor = "true")]
    pub excl_claims: Option<Vec<ClaimsSetClaims>>,
    #[cbor(tag = "6", cbor = "true")]
    pub keys: CasAndTasMap,
}

/// environment-group-list-map = {
///   ? environment-map => environment-map,
///   ? concise-swid-tag => abbreviated-swid-tag,
///   ? named-ta-store => named-ta-store,
/// }
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentGroupListMap {
    #[cbor(tag = "1", cbor = "true")]
    pub environment_map: Option<EnvironmentMap>,
    #[cbor(tag = "2", cbor = "true")]
    pub concise_swid_tag: Option<AbbreviatedSwidTag>,
    #[cbor(tag = "3", value = "Text")]
    pub named_ta_store: Option<String>,
}
