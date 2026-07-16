//! Map-based structs from the Entity Attestation Token (EAT) spec ([RFC 9711]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `Claims-Set` | [`ClaimsSetClaims`] / [`ClaimsSetClaimsCbor`] |
//! | `location-type` | [`LocationType`] / [`LocationTypeCbor`] |
//! | `sueids-type` | [`SueidsType`] / [`SueidsTypeCbor`] |
//!
//! [RFC 9711]: https://datatracker.ietf.org/doc/html/rfc9711

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{fmt, marker::PhantomData};

use ciborium::{cbor, value::Value};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

use cbor_derive::StructToMap;
use common::{
    GeneralProfile,
    tuple_map::{TupleMap, TupleMapCbor},
    *,
};

use crate::{arrays::*, cbor_specific::SubmodsMapCbor, choices::*, json_specific::SubmodsMap};

/// JSON encoding/decoding of `Claims-Set-Claims`, see [EAT Section 4.2].
///
/// Use [ClaimsSetClaimsCbor] for CBOR-encoded EATs.
///
/// ```text
/// string-or-uri = text
/// nonce-type = bstr .size (8..64)
/// oemid-pen = int
///
/// oemid-ieee = oemid-ieee-cbor
/// oemid-ieee-cbor = bstr .size 3
/// oemid-ieee-json = base64-url-text .size 4
///
/// oemid-random = oemid-random-cbor
/// oemid-random-cbor = bstr .size 16
/// oemid-random-json = base64-url-text .size 24
///
/// hardware-model-type = bytes .size (1..32)
///
/// ueid-type = bstr .size (7..33)
///
/// nonce-label            = 10
/// ueid-label             = 256
/// sueids-label           = 257
/// oemid-label            = 258
/// hardware-model-label   = 259
/// hardware-version-label = 260
/// uptime-label           = 261
/// oem-boot-label         = 262
/// debug-status-label     = 263
/// location-label         = 264
/// profile-label          = 265
/// submods-label          = 266
/// boot-count-label       = 267
/// boot-seed-label        = 268
/// dloas-label            = 269
/// sw-name-label          = 270
/// sw-version-label       = 271
/// manifests-label        = 272
/// measurements-label     = 273
/// measurement-results-label = 274
/// intended-use-label     = 275
///
/// iss-claim-label = 1
/// sub-claim-label = 2
/// aud-claim-label = 3
/// exp-claim-label = 4
/// nbf-claim-label = 5
/// iat-claim-label = 6
/// cti-claim-label = 7  ; jti in JWT: different name and text
/// $$Claims-Set-Claims //= (boot-count-label => uint)
/// $$Claims-Set-Claims //=  (boot-seed-label => binary-data)
/// $$Claims-Set-Claims //= ( debug-status-label => debug-status-type )
/// $$Claims-Set-Claims //= (
///     dloas-label => [ + dloa-type ]
/// )
/// $$Claims-Set-Claims //= (
///     hardware-model-label => hardware-model-type
/// )
/// $$Claims-Set-Claims //=  (
///     hardware-version-label => hardware-version-type
/// )
/// $$Claims-Set-Claims //= ( intended-use-label => intended-use-type )
/// $$Claims-Set-Claims //= (location-label => location-type)
/// $$Claims-Set-Claims //=
///     (nonce-label => nonce-type / [ 2* nonce-type ])
/// $$Claims-Set-Claims //= (
///     manifests-label => manifests-type
/// )
/// $$Claims-Set-Claims //= (
///     measurements-label => measurements-type
/// )
/// $$Claims-Set-Claims //= (
///     measurement-results-label =>
///         [ + measurement-results-group ] )
///
/// $$Claims-Set-Claims //= (
///     oemid-label => oemid-pen / oemid-ieee / oemid-random
/// )
/// $$Claims-Set-Claims //= (sueids-label => sueids-type)
/// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
///
/// $$Claims-Set-Claims //= (profile-label => general-uri / general-oid)
/// $$Claims-Set-Claims //= (oem-boot-label => bool)
/// $$Claims-Set-Claims //= (sw-name-label => tstr )
/// $$Claims-Set-Claims //= (sw-version-label => sw-version-type)
/// $$Claims-Set-Claims //= (ueid-label => ueid-type)
/// $$Claims-Set-Claims //= (uptime-label => uint)
///
/// $$Claims-Set-Claims //= ( iss-claim-label => string-or-uri  )
/// $$Claims-Set-Claims //= ( sub-claim-label => string-or-uri  )
/// $$Claims-Set-Claims //= ( aud-claim-label => string-or-uri  )
/// $$Claims-Set-Claims //= ( exp-claim-label => ~time )
/// $$Claims-Set-Claims //= ( nbf-claim-label => ~time )
/// $$Claims-Set-Claims //= ( iat-claim-label => ~time )
/// $$Claims-Set-Claims //= ( cti-claim-label => bytes )
///    Claims-Set = {
///        * $$Claims-Set-Claims
///        * Claim-Label .feature "extended-claims-label" => any
///    }
/// ```
/// [EAT Section 4.2]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ClaimsSetClaims {
    #[cbor(tag = "1", value = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[cbor(tag = "2", value = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    #[cbor(tag = "3", value = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    #[cbor(tag = "4", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<Time>,
    #[cbor(tag = "5", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<Time>,
    #[cbor(tag = "6", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<Time>,
    #[cbor(tag = "7", value = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cti: Option<Vec<u8>>,
    #[cbor(tag = "10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<NonceType>,
    #[cbor(tag = "267", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_count: Option<u64>,
    #[cbor(tag = "268", value = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_seed: Option<Vec<u8>>,
    #[cbor(tag = "263")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_status: Option<DebugStatusType>,
    #[cbor(tag = "269", value = "Array", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dloas: Option<Vec<DloaType>>,
    #[cbor(tag = "259", value = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_model: Option<Vec<u8>>,
    #[cbor(tag = "260", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_version: Option<HardwareVersionType>,
    #[cbor(tag = "275")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_use: Option<IntendedUseType>,
    #[cbor(tag = "264", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    #[cbor(tag = "265")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<GeneralProfile>,
    #[cbor(tag = "262", value = "Bool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oem_boot: Option<bool>,
    #[cbor(tag = "270", value = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_name: Option<String>,
    #[cbor(tag = "271", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<SwVersionType>,
    #[cbor(tag = "256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ueid: Option<UeidType>,
    #[cbor(tag = "261", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<u64>,
    #[cbor(tag = "272", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifests: Option<ManifestsType>,
    #[cbor(tag = "273", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<MeasurementsType>,
    #[cbor(tag = "274", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_results: Option<MeasurementResultsGroupArray>,
    #[cbor(tag = "258")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oemid: Option<Oemid>,
    #[cbor(tag = "257", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sueids: Option<TupleMap>,
    #[cbor(tag = "266", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submods: Option<SubmodsMap>,
    #[cbor(value = "Array", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<Tuple>>,
}

/// JSON encoding/decoding of `location-type`, see [EAT Section 4.2.10].
///
/// Use [LocationTypeCbor] for CBOR-encoded EATs.
///
/// ```text
/// location-type = {
///     latitude => number,
///     longitude => number,
///     ? altitude => number,
///     ? accuracy => number,
///     ? altitude-accuracy => number,
///     ? heading => number,
///     ? speed => number,
///     ? timestamp => ~time-int,
///     ? age => uint
/// }
///
/// latitude          = 1
/// longitude         = 2
/// altitude          = 3
/// accuracy          = 4
/// altitude-accuracy = 5
/// heading           = 6
/// speed             = 7
/// timestamp         = 8
/// age               = 9
/// ```
/// [EAT Section 4.2.10]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.10
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LocationType {
    #[cbor(tag = "1", value = "Float")]
    pub latitude: f64,
    #[cbor(tag = "2", value = "Float")]
    pub longitude: f64,
    #[cbor(tag = "3", value = "Float")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[cbor(tag = "4", value = "Float")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    #[cbor(tag = "5", value = "Float")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_accuracy: Option<f64>,
    #[cbor(tag = "6", value = "Float")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<f64>,
    #[cbor(tag = "7", value = "Float")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[cbor(tag = "8", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Time>,
    #[cbor(tag = "9", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u64>,
}

/// JSON encoding/decoding of `sueids-type`, see [EAT Section 4.2.2].
///
/// Use [SueidsTypeCbor] for CBOR-encoded EATs.
///
/// ```text
/// sueids-type = {
///     + tstr => ueid-type
/// }
/// ```
/// [EAT Section 4.2.2]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.2
pub struct SueidsType(pub TupleMap);

/// CBOR encoding/decoding of `sueids-type`, see [EAT Section 4.2.2].
///
/// Use [SueidsType] for JSON-encoded EATs.
///
/// ```text
/// sueids-type = {
///     + tstr => ueid-type
/// }
/// ```
/// [EAT Section 4.2.2]: https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.2
pub struct SueidsTypeCbor(pub TupleMapCbor);
