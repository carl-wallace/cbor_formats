//! Array-based structs from the Concise Reference Integrity Manifest (CoRIM) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{de::Error, de::Visitor};

use alloc::{vec, vec::Vec};

use crate::maps::*;
use alloc::format;
use alloc::string::{String, ToString};
use cbor_derive::StructToArray;

/// The `attest-key-triple-record` type is defined in [CoRIM Section 5.1.10].
///
/// ```text
/// attest-key-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.10]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.10
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AttestKeyTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `coswid-triple-record` type is defined in [CoRIM Section 5.1.12].
///
/// ```text
/// coswid-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.12]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.12
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoswidTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `domain-dependency-triple-record` type is defined in [CoRIM Section 5.1.11.2].
///
/// ```text
/// domain-dependency-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.11.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.11.2
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DomainDependencyTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `domain-membership-triple-record` type is defined in [CoRIM Section 5.1.11.1].
///
/// ```text
/// domain-membership-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.11.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.11.1
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DomainMembershipTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `endorsed-triple-record` type is defined in [CoRIM Section 5.1.6].
///
/// ```text
/// endorsed-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.6
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EndorsedTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `identity-triple-record` type is defined in [CoRIM Section 5.1.9].
///
/// ```text
/// identity-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.9]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.9
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct IdentityTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `reference-triple-record` type is defined in [CoRIM Section 5.1.5].
///
/// ```text
/// reference-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.5]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.5
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ReferenceTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    #[serde(rename = "environment")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    #[serde(rename = "measurements")]
    pub measurement_map: Vec<MeasurementMap>,
}

/// The `conditional-endorsement-triple-record` type is defined in [CoRIM Section 5.1.7].
///
/// ```text
/// conditional-endorsement-triple-record = [
///   conditions: [ + measurement-map ]
///   endorsements: [ + endorsed-triple-record ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.7]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.7
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConditionalEndorsementTripleRecord {
    #[cbor(value = "Array", cbor = "true")]
    pub conditions: Vec<MeasurementMap>,
    #[cbor(value = "Array", cbor = "true")]
    pub endorsements: Vec<EndorsedTripleRecord>,
}

/// The `series-record` type is defined in [CoRIM Section 5.1.8].
///
/// ```text
/// series-record = [selection: [+ measurement-map], addition: [+ measurement-map]]
/// ```
///
/// [CoRIM Section 5.1.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.8
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SeriesRecord {
    #[cbor(value = "Array", cbor = "true")]
    pub selection: Vec<MeasurementMap>,
    #[cbor(value = "Array", cbor = "true")]
    pub addition: Vec<MeasurementMap>,
}

/// The `conditional-endorsement-series-triple-record` type is defined in [CoRIM Section 5.1.8].
///
/// ```text
/// conditional-endorsement-series-triple-record = [
///   conditions: [ + measurement-map ]
///   series: [ + series-record ]
/// ]
/// ```
///
/// [CoRIM Section 5.1.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.8
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConditionalEndorsementSeriesTripleRecord {
    #[cbor(value = "Array", cbor = "true")]
    pub conditions: Vec<MeasurementMap>,
    #[cbor(value = "Array", cbor = "true")]
    pub series: Vec<SeriesRecord>,
}
