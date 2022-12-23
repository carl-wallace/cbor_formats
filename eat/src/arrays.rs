//! Array-based structs

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData, ops::Deref};

use ciborium::{cbor, value::Value};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use crate::cbor_specific::SelectorCbor;
use crate::choices::*;
use crate::json_specific::*;
use cbor_derive::StructToArray;
use common::{choices::*, *};

/// Provides access to JSON-Selector options suitable for inclusion in a Detached EAT Bundle
///
/// The EAT specification uses the `JSON-Selector` choice in two places: in the definition of
/// Submodule, in the definition of Detached-EAT-Bundle (as part of NestedToken). Where used within
/// a DEB, the  Detached-EAT-Bundle option MUST NOT be used.
///
/// Nested-Token = JSON-Selector
/// Nested-Token = CBOR-Nested-Token
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedToken(pub Box<SelectorForDeb>);
impl TryFrom<NestedTokenCbor> for NestedToken {
    type Error = String;
    fn try_from(value: NestedTokenCbor) -> Result<Self, Self::Error> {
        match value {
            NestedTokenCbor(SelectorCbor::JsonTokenInsideCborToken(s)) => {
                let sfd = SelectorForDeb {
                    token_type: JsonSelectorType::Jwt,
                    nested_token: JsonSelectorForDebValue::JwtMessage(s),
                };
                Ok(NestedToken(Box::new(sfd)))
            }
            NestedTokenCbor(SelectorCbor::CborTokenInsideCborToken(v)) => {
                let sfd = SelectorForDeb {
                    token_type: JsonSelectorType::Cbor,
                    nested_token: JsonSelectorForDebValue::CborTokenInsideJsonToken(
                        base64::encode(v),
                    ),
                };
                Ok(NestedToken(Box::new(sfd)))
            }
            NestedTokenCbor(SelectorCbor::DetachedSubmoduleDigest(dsm)) => {
                //todo unwrap
                let sfd = SelectorForDeb {
                    token_type: JsonSelectorType::Digest,
                    nested_token: JsonSelectorForDebValue::DetachedSubmoduleDigest(
                        dsm.try_into().unwrap(),
                    ),
                };
                Ok(NestedToken(Box::new(sfd)))
            }
        }
    }
}
impl TryFrom<&NestedTokenCbor> for NestedToken {
    type Error = String;
    fn try_from(value: &NestedTokenCbor) -> Result<Self, Self::Error> {
        match value {
            NestedTokenCbor(SelectorCbor::JsonTokenInsideCborToken(s)) => {
                let sfd = SelectorForDeb {
                    token_type: JsonSelectorType::Jwt,
                    nested_token: JsonSelectorForDebValue::JwtMessage(s.clone()),
                };
                Ok(NestedToken(Box::new(sfd)))
            }
            NestedTokenCbor(SelectorCbor::CborTokenInsideCborToken(v)) => {
                let sfd = SelectorForDeb {
                    token_type: JsonSelectorType::Cbor,
                    nested_token: JsonSelectorForDebValue::CborTokenInsideJsonToken(
                        base64::encode(v),
                    ),
                };
                Ok(NestedToken(Box::new(sfd)))
            }
            NestedTokenCbor(SelectorCbor::DetachedSubmoduleDigest(dsm)) => {
                //todo unwrap
                let sfd = SelectorForDeb {
                    token_type: JsonSelectorType::Digest,
                    nested_token: JsonSelectorForDebValue::DetachedSubmoduleDigest(
                        dsm.try_into().unwrap(),
                    ),
                };
                Ok(NestedToken(Box::new(sfd)))
            }
        }
    }
}

/// Nested-Token = JSON-Selector
/// Nested-Token = CBOR-Nested-Token
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedTokenCbor(pub SelectorCbor);
impl TryFrom<Value> for NestedTokenCbor {
    type Error = String;
    fn try_from(_value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&Value> for NestedTokenCbor {
    type Error = String;
    fn try_from(_value: &Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<NestedToken> for NestedTokenCbor {
    type Error = String;
    fn try_from(value: NestedToken) -> Result<Self, Self::Error> {
        let js = value.0.deref();
        match &js.nested_token {
            JsonSelectorForDebValue::JwtMessage(s) => Ok(NestedTokenCbor(
                SelectorCbor::JsonTokenInsideCborToken(s.to_string()),
            )),
            JsonSelectorForDebValue::CborTokenInsideJsonToken(s) => match base64::decode(s) {
                Ok(v) => Ok(NestedTokenCbor(SelectorCbor::CborTokenInsideCborToken(v))),
                Err(e) => Err(format!("{e}")),
            },
            JsonSelectorForDebValue::DetachedSubmoduleDigest(s) => {
                let dsd_cbor: Result<DetachedSubmoduleDigestCbor, String> = s.try_into();
                match dsd_cbor {
                    Ok(dc) => Ok(NestedTokenCbor(SelectorCbor::DetachedSubmoduleDigest(dc))),
                    Err(e) => Err(e),
                }
            }
        }
    }
}
impl TryFrom<&NestedToken> for NestedTokenCbor {
    type Error = String;
    fn try_from(value: &NestedToken) -> Result<Self, Self::Error> {
        let js = value.0.deref();
        match &js.nested_token {
            JsonSelectorForDebValue::JwtMessage(s) => Ok(NestedTokenCbor(
                SelectorCbor::JsonTokenInsideCborToken(s.to_string()),
            )),
            JsonSelectorForDebValue::CborTokenInsideJsonToken(s) => match base64::decode(s) {
                Ok(v) => Ok(NestedTokenCbor(SelectorCbor::CborTokenInsideCborToken(v))),
                Err(e) => Err(format!("{e}")),
            },
            JsonSelectorForDebValue::DetachedSubmoduleDigest(s) => {
                let dsd_cbor: Result<DetachedSubmoduleDigestCbor, String> = s.try_into();
                match dsd_cbor {
                    Ok(dc) => Ok(NestedTokenCbor(SelectorCbor::DetachedSubmoduleDigest(dc))),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

/// json-wrapped-claims-set = base64-url-text
/// cbor-wrapped-claims-set = bstr .cbor Claims-Set
/// JC<json-wrapped-claims-set, cbor-wrapped-claims-set>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WrappedClaimsSet(pub String);
impl TryFrom<WrappedClaimsSetCbor> for WrappedClaimsSet {
    type Error = String;
    fn try_from(value: WrappedClaimsSetCbor) -> Result<Self, Self::Error> {
        Ok(WrappedClaimsSet(base64::encode(value.0)))
    }
}
impl TryFrom<&WrappedClaimsSetCbor> for WrappedClaimsSet {
    type Error = String;
    fn try_from(value: &WrappedClaimsSetCbor) -> Result<Self, Self::Error> {
        Ok(WrappedClaimsSet(base64::encode(value.0.clone())))
    }
}

/// JC<json-wrapped-claims-set, cbor-wrapped-claims-set>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WrappedClaimsSetCbor(pub Vec<u8>);
impl TryFrom<Value> for WrappedClaimsSetCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.as_bytes() {
            Some(b) => Ok(WrappedClaimsSetCbor(b.clone())),
            None => Err("Failed to parse Value as WrappedClaimsSetCbor".to_string()),
        }
    }
}
impl TryFrom<&Value> for WrappedClaimsSetCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.as_bytes() {
            Some(b) => Ok(WrappedClaimsSetCbor(b.clone())),
            None => Err("Failed to parse Value as WrappedClaimsSetCbor".to_string()),
        }
    }
}
impl TryFrom<WrappedClaimsSet> for WrappedClaimsSetCbor {
    type Error = String;
    fn try_from(value: WrappedClaimsSet) -> Result<Self, Self::Error> {
        match base64::decode(value.0) {
            Ok(v) => Ok(WrappedClaimsSetCbor(v)),
            Err(e) => Err(format!("{e}")),
        }
    }
}
impl TryFrom<&WrappedClaimsSet> for WrappedClaimsSetCbor {
    type Error = String;
    fn try_from(value: &WrappedClaimsSet) -> Result<Self, Self::Error> {
        match base64::decode(&value.0) {
            Ok(v) => Ok(WrappedClaimsSetCbor(v)),
            Err(e) => Err(format!("{e}")),
        }
    }
}

// Detached-EAT-Bundle = [
//     main-token : Nested-Token,
//     detached-claims-sets: {
//         + tstr => JC<json-wrapped-claims-set,
//                      cbor-wrapped-claims-set>
//     }
// ]
//
// json-wrapped-claims-set = base64-url-text
//
// cbor-wrapped-claims-set = bstr .cbor Claims-Set
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DetachedEatBundle {
    #[cbor(cbor = "true")]
    pub main_token: NestedToken,
    #[cbor(value = "Array", cbor = "true")]
    pub detached_claims_set: Vec<WrappedClaimsSet>,
}

/// Detached-Submodule-Digest = [
///    hash-algorithm : text / int,
///    digest         : binary-data
/// ]
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DetachedSubmoduleDigest {
    pub hash_algorithm: TextOrInt,
    #[cbor(value = "Bytes")]
    pub digest: Vec<u8>,
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
pub struct ManifestsType(pub Vec<ManifestFormat>);

/// manifests-type = [+ manifest-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ManifestsTypeCbor(pub Vec<ManifestFormatCbor>);

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
pub struct MeasurementsType(pub Vec<MeasurementsFormat>);

/// measurements-type = [+ measurements-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementsTypeCbor(pub Vec<MeasurementsFormatCbor>);

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

/// measurements-type = [+ measurements-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementResultsGroupArray(pub Vec<MeasurementResultsGroup>);

/// measurements-type = [+ measurements-format]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MeasurementResultsGroupArrayCbor(pub Vec<MeasurementResultsGroupCbor>);
impl MeasurementResultsGroupArrayCbor {
    /// initialize a new MeasurementResultsGroupArrayCbor
    pub fn new(mr: Vec<MeasurementResultsGroupCbor>) -> MeasurementResultsGroupArrayCbor {
        MeasurementResultsGroupArrayCbor(mr)
    }
}

// todo closure error handling
impl TryFrom<&Value> for MeasurementResultsGroupArrayCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(v) => Ok(MeasurementResultsGroupArrayCbor(
                v.iter()
                    .map(|m| MeasurementResultsGroupCbor::try_from(m).unwrap())
                    .collect(),
            )),
            _ => Err("Failed to parse value as an array for EnvironmentGroupListCbor".to_string()),
        }
    }
}

#[allow(unused_variables)]
impl TryFrom<&MeasurementResultsGroupArray> for MeasurementResultsGroupArrayCbor {
    type Error = String;
    fn try_from(value: &MeasurementResultsGroupArray) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<MeasurementResultsGroupCbor>::try_into(v) {
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
impl TryFrom<&MeasurementResultsGroupArrayCbor> for MeasurementResultsGroupArray {
    type Error = String;
    fn try_from(value: &MeasurementResultsGroupArrayCbor) -> Result<Self, Self::Error> {
        let mut retval = Self(vec![]);
        for v in &value.0 {
            match TryInto::<MeasurementResultsGroup>::try_into(v) {
                Ok(v) => retval.0.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(retval)
    }
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
