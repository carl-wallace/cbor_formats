//! Array-based structs from the Entity Attestation Token (EAT) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use serde::ser::Error as OtherError;

use crate::choices::*;
use cbor_derive::StructToArray;
use common::choices::*;
use common::TextOrBinary;
use common::TextOrInt;
use common::Uri;

// Detached-EAT-Bundle = [
//     main-token : Nested-Token,
//     detached-claims-sets: {
//         + tstr => cbor-wrapped-claims-set
//     }
// ]

// Detached-Submodule-Digest = [
//    hash-algorithm : text / int,
//    digest         : binary-data
// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DetachedSubmoduleDigest {
    pub hash_algorithm: TextOrInt,
    #[cbor(value = "Bytes")]
    pub dloa_platform_label: Vec<u8>,
}

// dloa-type = [
//     dloa_registrar: general-uri
//     dloa_platform_label: text
//     ? dloa_application_label: text
// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DloaType {
    #[cbor(value = "Text")]
    pub dloa_registrar: Uri,
    #[cbor(value = "Text")]
    pub dloa_platform_label: String,
    #[cbor(value = "Text")]
    pub dloa_application_label: Option<String>,
}

// hardware-version-type = [
//     version:  tstr,
//     ? scheme:  $version-scheme
// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HardwareVersionType {
    #[cbor(value = "Text")]
    pub version: String,
    #[cbor(cbor = "true")]
    pub scheme: Option<VersionScheme>,
}

// individual-result = [
//     results-id: tstr / binary-data,
//     result:     result-type,
// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct IndividualResult {
    pub results_id: TextOrBinary,
    pub result: ResultType,
}

// JSON-Selector = [
//    type : $JSON-Selector-Type,
//    nested-token : $JSON-Selector-Value
// ]

// manifests-type = [+ manifest-format]
//
// manifest-format = [
//     content-type:   coap-content-format,
//     content-format: $manifest-body-cbor
// ]

// measurements-type = [+ measurements-format]
//
// measurements-format = [
//     content-type:   coap-content-format,
//     content-format: $measurements-body-cbor
// ]

// measurement-results-group = [
//     measurement-system: tstr,
//     measurement-results: [ + individual-result ]
// ]
//
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementResultsGroup {
    #[cbor(value = "Text")]
    pub measurement_system: String,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_results: Vec<IndividualResult>,
}

// sw-version-type = [
//     version:  tstr
//     ? scheme:  $version-scheme ; As defined by CoSWID
// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SwVersionType {
    #[cbor(value = "Text")]
    pub version: String,
    #[cbor(cbor = "true")]
    pub scheme: Option<VersionScheme>,
}
