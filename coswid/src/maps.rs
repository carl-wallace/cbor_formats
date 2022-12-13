//! Map-based structs from the Concise Software Identification Tags (CoSWID) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    __private::size_hint,
    de::{Error, MapAccess, Visitor},
};

//use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

use crate::choices::*;
use cbor_derive::StructToMap;
use cbor_derive::StructToOneOrMore;
use common::arrays::*;
use common::choices::*;
use common::*;
use serde::ser::Error as OtherError;

// ; concise-swig-tag map indices (culled from global map member)
// tag-id = 0
// tag-version = 12
// corpus = 8
// patch = 9
// supplemental = 11
// software-name = 1
// software-version = 13
// version-scheme = 14
// media = 10
// software-meta = 5
// entity = 2
// link = 4
// evidence = 3
// payload = 6

/// The `concise-swid-tag` type is defined in [CoSWID Section 2.3].
///
/// ```text
/// concise-swid-tag = {
///   tag-id => text / bstr .size 16,
///   tag-version => integer,
///   ? corpus => bool,
///   ? patch => bool,
///   ? supplemental => bool,
///   software-name => text,
///   ? software-version => text,
///   ? version-scheme => $version-scheme,
///   ? media => text,
///   ? software-meta => one-or-more<software-meta-entry>,
///   entity => one-or-more<entity-entry>,
///   ? link => one-or-more<link-entry>,
///   ? payload-or-evidence,
///   * $$coswid-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.3]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConciseSwidTag {
    #[cbor(tag = "0")]
    pub tag_id: TextOrBinary,
    #[cbor(tag = "12", value = "Integer")]
    pub tag_version: i64,
    #[cbor(tag = "8", value = "Bool")]
    pub corpus: Option<bool>,
    #[cbor(tag = "9", value = "Bool")]
    pub patch: Option<bool>,
    #[cbor(tag = "11", value = "Bool")]
    pub supplemental: Option<bool>,
    #[cbor(tag = "1", value = "Text")]
    pub software_name: String,
    #[cbor(tag = "13", value = "Text")]
    pub software_version: Option<String>,
    #[cbor(tag = "14", cbor = "true")]
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
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

// directory-entry = {
//    ? key => bool,
//    ? location => text,
//    fs-name => text,
//    ? root => text,
//   ? path-elements => { path-elements-group },
//   * $$directory-extension,
//   ? lang => text,
//   global-attributes,
// }

/// The `directory-entry` type is defined in [CoSWID Section 2.9.2].
///
/// ```text
/// directory-entry = {
///   filesystem-item,
///   ? path-elements => { path-elements-group },
///   * $$directory-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.9.2]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.2
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DirectoryEntry {
    #[cbor(tag = "22", value = "Bool")]
    pub key: Option<bool>,
    #[cbor(tag = "23", value = "Text")]
    pub location: Option<String>,
    #[cbor(tag = "24", value = "Text")]
    pub fs_name: String,
    #[cbor(tag = "25", value = "Text")]
    pub root: Option<String>,
    //todo handle recursion via Box
    // #[cbor(tag = "26", value = "Text")]
    // pub path_elements: Option<Box<PathElementsGroup>>,
    //   * $$directory-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `entity-entry` type is defined in [CoSWID Section 2.6].
///
/// ```text
/// entity-entry = {
///   entity-name => text,
///   ? reg-id => any-uri,
///   role => one-or-more<$role>,
///   ? thumbprint => hash-entry,
///   * $$entity-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.6]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.6
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EntityEntry {
    #[cbor(tag = "31", value = "Text")]
    pub entity_name: String,
    #[cbor(tag = "32", value = "Text")]
    pub reg_id: Option<Uri>,
    #[cbor(tag = "33")]
    pub role: OneOrMoreRole,
    #[cbor(tag = "34", cbor = "true")]
    pub thumbprint: Option<HashEntry>,
    //   * $$entity-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `evidence-entry` type is defined in [CoSWID Section 2.9.4].
///
/// ```text
/// evidence-entry = {
///   resource-collection,
///   ? date => integer-time,
///   ? device-id => text,
///   * $$evidence-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.9.4]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EvidenceEntry {
    #[cbor(tag = "16", cbor = "true")]
    pub directory: Option<OneOrMoreDirectoryEntry>,
    #[cbor(tag = "17", cbor = "true")]
    pub file: Option<OneOrMoreFileEntry>,
    #[cbor(tag = "18", cbor = "true")]
    pub process: Option<OneOrMoreProcessEntry>,
    #[cbor(tag = "19", cbor = "true")]
    pub resource: Option<OneOrMoreResourceEntry>,
    #[cbor(tag = "35", cbor = "true")]
    pub date: Option<Time>,
    #[cbor(tag = "36", value = "Text")]
    pub device_id: Option<String>,
    //   * $$evidence-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `file-entry` type is defined in [CoSWID Section 2.9.2].
///
/// ```text
/// file-entry = {
///   filesystem-item,
///   ? size => uint,
///   ? file-version => text,
///   ? hash => hash-entry,
///   * $$file-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.9.2]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.2
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct FileEntry {
    //todo filesystem-item
    #[cbor(tag = "20", value = "Integer")]
    pub size: Option<i64>,
    #[cbor(tag = "21", value = "Text")]
    pub file_version: Option<String>,
    #[cbor(tag = "7", cbor = "true")]
    pub hash: HashEntry,
    //   * $$file-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `link-entry` type is defined in [CoSWID Section 2.7].
///
/// ```text
/// link-entry = {
///   ? artifact => text,
///   href => any-uri,
///   ? media => text,
///   ? ownership => $ownership,
///   rel => $rel,
///   ? media-type => text,
///   ? use => $use,
///   * $$link-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.7]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.7
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LinkEntry {
    #[cbor(tag = "37", value = "Text")]
    pub artifact: Option<String>,
    #[cbor(tag = "38", value = "Text")]
    pub href: Uri,
    #[cbor(tag = "10", value = "Text")]
    pub media: Option<String>,
    #[cbor(tag = "39")]
    pub ownership: Option<Ownership>,
    #[cbor(tag = "40")]
    pub rel: Rel,
    #[cbor(tag = "41", value = "Text")]
    pub media_type: Option<String>,
    #[cbor(tag = "42")]
    pub use_choice: Option<UseChoice>,
    //   * $$link-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `path-elements-group` type is defined in [CoSWID Section 2.9.2].
///
/// ```text
/// { path-elements-group }
/// path-elements-group = ( ? directory => one-or-more<directory-entry>,
///                         ? file => one-or-more<file-entry>,
///                       )
/// ```
///
/// [CoSWID Section 2.9.2]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PathElementsGroup {
    #[cbor(tag = "16", cbor = "true")]
    pub directory: Option<OneOrMoreDirectoryEntry>,
    #[cbor(tag = "17", cbor = "true")]
    pub file: Option<OneOrMoreFileEntry>,
}

/// The `payload-entry` type is defined in [CoSWID Section 2.9.3].
///
/// ```text
/// payload-entry = {
///   resource-collection,
///   * $$payload-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.9.3]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PayloadEntry {
    #[cbor(tag = "16", cbor = "true")]
    pub directory: Option<OneOrMoreDirectoryEntry>,
    #[cbor(tag = "17", cbor = "true")]
    pub file: Option<OneOrMoreFileEntry>,
    #[cbor(tag = "18", cbor = "true")]
    pub process: Option<OneOrMoreProcessEntry>,
    #[cbor(tag = "19", cbor = "true")]
    pub resource: Option<OneOrMoreResourceEntry>,
    //   * $$payload-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `process-entry` type is defined in [CoSWID Section 2.9.2].
///
/// ```text
/// process-entry = {
///   process-name => text,
///   ? pid => integer,
///   * $$process-extension,
///   global-attributes,
/// }
///
/// [CoSWID Section 2.9.2]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.2
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ProcessEntry {
    #[cbor(tag = "27", value = "Text")]
    pub process_name: String,
    #[cbor(tag = "28", value = "Integer")]
    pub pin: i64,
    //   * process-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `resource-entry` type is defined in [CoSWID Section 2.9.2].
///
/// ```text
/// resource-entry = {
///   type => text,
///   * $$resource-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.9.2]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.9.2
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ResourceEntry {
    #[cbor(tag = "29", value = "Text")]
    pub resource_entry_type: String,
    //   * $$resource-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// The `software-meta-entry` type is defined in [CoSWID Section 2.8].
///
/// ```text
/// software-meta-entry = {
///   ? activation-status => text,
///   ? channel-type => text,
///   ? colloquial-version => text,
///   ? description => text,
///   ? edition => text,
///   ? entitlement-data-required => bool,
///   ? entitlement-key => text,
///   ? generator => text,
///   ? persistent-id => text,
///   ? product => text,
///   ? product-family => text,
///   ? revision => text,
///   ? summary => text,
///   ? unspsc-code => text,
///   ? unspsc-version => text,
///   * $$software-meta-extension,
///   global-attributes,
/// }
/// ```
///
/// [CoSWID Section 2.8]: https://datatracker.ietf.org/doc/html/draft-ietf-sacm-coswid-22#section-2.8
#[derive(Clone, Debug, PartialEq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SoftwareMetaEntry {
    #[cbor(tag = "43", value = "Text")]
    pub activation_status: Option<String>,
    #[cbor(tag = "44", value = "Text")]
    pub channel_type: Option<String>,
    #[cbor(tag = "45", value = "Text")]
    pub colloquial_version: Option<String>,
    #[cbor(tag = "46", value = "Text")]
    pub decription: Option<String>,
    #[cbor(tag = "47", value = "Text")]
    pub edition: Option<String>,
    #[cbor(tag = "48", value = "Bool")]
    pub entitlement_data_required: Option<bool>,
    #[cbor(tag = "49", value = "Text")]
    pub entitlement_key: Option<String>,
    #[cbor(tag = "50", value = "Text")]
    pub generator: Option<String>,
    #[cbor(tag = "51", value = "Text")]
    pub persistent_id: Option<String>,
    #[cbor(tag = "52", value = "Text")]
    pub product: Option<String>,
    #[cbor(tag = "53", value = "Text")]
    pub product_family: Option<String>,
    #[cbor(tag = "54", value = "Text")]
    pub revision: Option<String>,
    #[cbor(tag = "55", value = "Text")]
    pub summary: Option<String>,
    #[cbor(tag = "56", value = "Text")]
    pub unspsc_code: Option<String>,
    #[cbor(tag = "57", value = "Text")]
    pub unspsc_version: Option<String>,
    //   * software-meta-extension,
    #[cbor(tag = "15", value = "Text")]
    pub lang: Option<String>,
    //   global-attributes,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}
