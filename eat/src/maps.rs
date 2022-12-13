//! Map-based structs from the Entity Attestation Token (EAT) spec

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    __private::size_hint,
    de::{MapAccess, Visitor},
};

use crate::arrays::*;
use crate::choices::*;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use cbor_derive::StructToMap;
use common::tuple_map::{TupleMap, TupleMapCbor};
use common::*;
use corim::choices::ProfileTypeChoice;
use serde::de::Error;
use serde::ser::Error as OtherError;

// $$Claims-Set-Claims //= (boot-count-label => uint)
// $$Claims-Set-Claims //=  (boot-seed-label => binary-data)
// $$Claims-Set-Claims //= ( debug-status-label => debug-status-type )
// $$Claims-Set-Claims //= (
//     dloas-label => [ + dloa-type ]
// )
// $$Claims-Set-Claims //= (
//     hardware-model-label => hardware-model-type
// )
// $$Claims-Set-Claims //=  (
//     hardware-version-label => hardware-version-type
// )
// $$Claims-Set-Claims //= ( intended-use-label => intended-use-type )
// $$Claims-Set-Claims //= (location-label => location-type)
// $$Claims-Set-Claims //=
//     (nonce-label => nonce-type / [ 2* nonce-type ])

// begin todo
// $$Claims-Set-Claims //= (
//     manifests-label => manifests-type
// )
// $$Claims-Set-Claims //= (
//     measurement-results-label =>
//         [ + measurement-results-group ] )
//
// $$Claims-Set-Claims //= (
//     measurements-label => measurements-type
// )
// $$Claims-Set-Claims //= (
//     oemid-label => oemid-pen / oemid-ieee / oemid-random
// )
// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
// $$Claims-Set-Claims //= (sueids-label => sueids-type)
// end todo

// $$Claims-Set-Claims //= (profile-label => general-uri / general-oid)
// $$Claims-Set-Claims //= (secure-boot-label => bool)
// $$Claims-Set-Claims //= (sw-name-label => tstr )
// $$Claims-Set-Claims //= (sw-version-label => sw-version-type)
// $$Claims-Set-Claims //= (ueid-label => ueid-type)
// $$Claims-Set-Claims //= (uptime-label => uint)

// $$Claims-Set-Claims //= ( iss-claim-label => string-or-uri  )
// $$Claims-Set-Claims //= ( sub-claim-label => string-or-uri  )
// $$Claims-Set-Claims //= ( aud-claim-label => string-or-uri  )
// $$Claims-Set-Claims //= ( exp-claim-label => ~time )
// $$Claims-Set-Claims //= ( nbf-claim-label => ~time )
// $$Claims-Set-Claims //= ( iat-claim-label => ~time )
// $$Claims-Set-Claims //= ( cti-claim-label => bytes )
//    Claims-Set = {
//        * $$Claims-Set-Claims
//        * Claim-Label .feature "extended-claims-label" => any
//    }
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ClaimsSetClaims {
    #[cbor(tag = "1", value = "Text")]
    pub iss: Option<String>,
    #[cbor(tag = "2", value = "Text")]
    pub sub: Option<String>,
    #[cbor(tag = "3", value = "Text")]
    pub aud: Option<String>,
    #[cbor(tag = "4", cbor = "true")]
    pub exp: Option<Time>,
    #[cbor(tag = "5", cbor = "true")]
    pub nbf: Option<Time>,
    #[cbor(tag = "6", cbor = "true")]
    pub iat: Option<Time>,
    #[cbor(tag = "7", value = "Bytes")]
    pub cti: Option<Vec<u8>>,
    #[cbor(tag = "10")]
    pub nonce: Option<NonceType>,
    #[cbor(tag = "276", value = "Integer")]
    pub boot_count: Option<u64>,
    #[cbor(tag = "268", value = "Bytes")]
    pub boot_seed: Option<Vec<u8>>,
    #[cbor(tag = "263")]
    pub debug_status: Option<DebugStatusType>,
    #[cbor(tag = "270", value = "Array", cbor = "true")]
    pub dloas: Option<Vec<DloaType>>,
    #[cbor(tag = "259", value = "Bytes")]
    pub hardware_model: Option<Vec<u8>>,
    #[cbor(tag = "260", cbor = "true")]
    pub hardware_version: Option<HardwareVersionType>,
    #[cbor(tag = "269")]
    pub intended_use: Option<IntendedUseType>,
    #[cbor(tag = "264", cbor = "true")]
    pub location: Option<LocationType>,
    #[cbor(tag = "265")]
    pub profile: Option<ProfileTypeChoice>,
    #[cbor(tag = "262", value = "Bool")]
    pub secure_boot: Option<bool>,
    #[cbor(tag = "271", value = "Text")]
    pub sw_name: Option<String>,
    #[cbor(tag = "272", cbor = "true")]
    pub sw_version: Option<SwVersionType>,
    #[cbor(tag = "256")]
    pub ueid: Option<UeidType>,
    #[cbor(tag = "267", value = "Integer")]
    pub uptime: Option<u64>,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

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
#[derive(Clone, Debug, Eq, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LocationType {
    #[cbor(tag = "1", value = "Integer")]
    pub latitude: u64,
    #[cbor(tag = "2", value = "Integer")]
    pub longitude: u64,
    #[cbor(tag = "3", value = "Integer")]
    pub altitude: Option<u64>,
    #[cbor(tag = "4", value = "Integer")]
    pub accuracy: Option<u64>,
    #[cbor(tag = "5", value = "Integer")]
    pub altitude_accuracy: Option<u64>,
    #[cbor(tag = "6", value = "Integer")]
    pub heading: Option<u64>,
    #[cbor(tag = "7", value = "Integer")]
    pub speed: Option<u64>,
    #[cbor(tag = "8", cbor = "true")]
    pub timestamp: Option<Time>,
    #[cbor(tag = "9", value = "Integer")]
    pub age: Option<u64>,
}

/// sueids-type = {
///     + tstr => ueid-type
/// }
pub struct SueidsType(TupleMap);

/// sueids-type = {
///     + tstr => ueid-type
/// }
pub struct SueidsTypeCbor(TupleMapCbor);
