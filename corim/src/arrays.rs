//! Array-based structs from the Concise Reference Integrity Manifest (CoRIM) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use alloc::{vec, vec::Vec};

use crate::choices::*;
use crate::maps::*;
use cbor_derive::StructToArray;
use common::TextOrBinary;

// todo - switch to CryptoKeyTypeChoice when corim repo catches up to the spec
/// The `attest-key-triple-record` type is defined in [CoRIM Section 3.1.4.5].
///
/// ```text
/// attest-key-triple-record = [
///   environment-map
///   [ + $crypto-key-type-choice ]
/// ]
/// ```
///
/// [CoRIM Section 3.1.4.5]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.5
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AttestKeyTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub crypto_keys: Vec<VerificationKeyMap>,
    // pub crypto_keys: Option<Vec<CryptoKeyTypeChoice>>,
}

/// The `coswid-triple-record` type is defined in [CoRIM Section 3.1.4.8].
///
/// ```text
///    coswid-triple-record = [
///      environment-map
///      [ + concise-swid-tag-id ]
///    ]
/// ```
///
/// [CoRIM Section 3.1.4.8]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.8
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoswidTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array")]
    pub coswid_tags: Vec<TextOrBinary>,
}

/// The `domain-dependency-triple-record` type is defined in [CoRIM Section 3.1.4.6].
///
/// ```text
/// domain-dependency-triple-record = [
///  $domain-type-choice
///  [ + $domain-type-choice ]
/// ]
/// ```
///
/// [CoRIM Section 3.1.4.6]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.6
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DomainDependencyTripleRecord {
    pub domain_type_choice: DomainTypeChoice,
    #[cbor(value = "Array")]
    pub domain_type_choices: Vec<DomainTypeChoice>,
}

/// The `endorsed-triple-record` type is defined in [CoRIM Section 3.1.4.3].
///
/// ```text
/// endorsed-triple-record = [
///   environment-map
///   [ + measurement-map ]
/// ]
/// ```
///
/// [CoRIM Section 3.1.4.3]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.3
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EndorsedTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurement_map: Vec<MeasurementMap>,
}

// todo - switch to CryptoKeyTypeChoice when corim repo catches up to the spec
/// The `identity-triple-record` type is defined in [CoRIM Section 3.1.4.4].
///
/// ```text
/// identity-triple-record = [
///   environment-map
///   [ + $crypto-key-type-choice ]
/// ]
/// ```
///
/// [CoRIM Section 3.1.4.4]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.4
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct IdentityTripleRecord {
    #[cbor(value = "Map", cbor = "true")]
    pub environment_map: EnvironmentMap,
    #[cbor(value = "Array", cbor = "true")]
    pub crypto_keys: Vec<VerificationKeyMap>,
    // pub crypto_keys: Vec<CryptoKeyTypeChoice>,
}

/// The `reference-triple-record` type is defined in [CoRIM Section 3.1.4.2].
///
/// ```text
/// reference-triple-record = [
///   environment-map ; target environment
///   [ + measurement-map ] ; reference measurements
/// ]
/// ```
///
/// [CoRIM Section 3.1.4.2]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-3.1.4.2
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
