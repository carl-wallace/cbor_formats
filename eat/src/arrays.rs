//! Array-based structs from the Entity Attestation Token (EAT) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use serde::ser::Error as OtherError;

use crate::choices::*;
use cbor_derive::StructToArray;
use common::choices::*;
use common::TextOrInt;
use common::Uri;
use common::{CoapContentFormat, TextOrBinary};

// Detached-EAT-Bundle = [
//     main-token : Nested-Token,
//     detached-claims-sets: {
//         + tstr => cbor-wrapped-claims-set
//     }
// ]

/// Detached-Submodule-Digest = [
///    hash-algorithm : text / int,
///    digest         : binary-data
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DetachedSubmoduleDigest {
    pub hash_algorithm: TextOrInt,
    #[cbor(value = "Bytes")]
    pub dloa_platform_label: Vec<u8>,
}

/// dloa-type = [
///     dloa_registrar: general-uri
///     dloa_platform_label: text
///     ? dloa_application_label: text
/// ]
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

/// hardware-version-type = [
///     version:  tstr,
///     ? scheme:  $version-scheme
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HardwareVersionType {
    #[cbor(value = "Text")]
    pub version: String,
    #[cbor(cbor = "true")]
    pub scheme: Option<VersionScheme>,
}

/// individual-result = [
///     results-id: tstr / binary-data,
///     result:     result-type,
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct IndividualResult {
    pub results_id: TextOrBinary,
    pub result: ResultType,
}

/// manifests-type = [+ manifest-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ManifestsType(Vec<ManifestFormat>);

/// manifests-type = [+ manifest-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ManifestsTypeCbor(Vec<ManifestFormatCbor>);

// todo closure error handling
impl TryFrom<&Value> for ManifestsTypeCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => Ok(ManifestsTypeCbor(
                v.iter()
                    .map(|m| ManifestFormatCbor::try_from(m).unwrap())
                    .collect(),
            )),
            _ => Err("Failed to parse value as an array for EnvironmentGroupListCbor".to_string()),
        }
    }
}

#[allow(unused_variables)]
impl TryFrom<&ManifestsType> for ManifestsTypeCbor {
    type Error = String;
    fn try_from(value: &ManifestsType) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<ManifestFormatCbor>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}
#[allow(unused_variables)]
impl TryFrom<&ManifestsTypeCbor> for ManifestsType {
    type Error = String;
    fn try_from(value: &ManifestsTypeCbor) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<ManifestFormat>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}

/// The $manifest-body-json and $manifest-body-cbor distill down to text or binary,
/// so the TextOrBinary enum is used instead of types specific to the manifest-format
/// claim. The type is indicated by the content-type field in all cases, so the extra
/// complexity does not provide much value (and TextOrBinary provides for extensibility).
///
/// manifest-format = [
///     content-type:   coap-content-format,
///     content-format: JC< $manifest-body-json,
///                         $manifest-body-cbor >
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ManifestFormat {
    #[cbor(value = "Integer")]
    pub content_type: CoapContentFormat,
    pub content_format: TextOrBinary,
}

/// measurements-type = [+ measurements-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementsType(Vec<MeasurementsFormat>);

/// measurements-type = [+ measurements-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementsTypeCbor(Vec<MeasurementsFormatCbor>);

// todo closure error handling
impl TryFrom<&Value> for MeasurementsTypeCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => Ok(MeasurementsTypeCbor(
                v.iter()
                    .map(|m| MeasurementsFormatCbor::try_from(m).unwrap())
                    .collect(),
            )),
            _ => Err("Failed to parse value as an array for EnvironmentGroupListCbor".to_string()),
        }
    }
}

#[allow(unused_variables)]
impl TryFrom<&MeasurementsType> for MeasurementsTypeCbor {
    type Error = String;
    fn try_from(value: &MeasurementsType) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<MeasurementsFormatCbor>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}
#[allow(unused_variables)]
impl TryFrom<&MeasurementsTypeCbor> for MeasurementsType {
    type Error = String;
    fn try_from(value: &MeasurementsTypeCbor) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<MeasurementsFormat>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
}

///
/// For the moment, the $measurements-body-cbor socket is not supported and instead
/// content-format is represented as TextOrBinary (to facilitate use of text via an extension).
///
/// measurements-format = [
///     content-type:   coap-content-format,
///     content-format: $measurements-body-cbor
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementsFormat {
    #[cbor(value = "Integer")]
    pub content_type: CoapContentFormat,
    pub content_format: TextOrBinary,
}

/// measurement-results-group = [
///     measurement-system: tstr,
///     measurement-results: [ + individual-result ]
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementResultsGroup {
    #[cbor(value = "Text")]
    pub measurement_system: String,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_results: Vec<IndividualResult>,
}

/// sw-version-type = [
///     version:  tstr
///     ? scheme:  $version-scheme ; As defined by CoSWID
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SwVersionType {
    #[cbor(value = "Text")]
    pub version: String,
    #[cbor(cbor = "true")]
    pub scheme: Option<VersionScheme>,
}
