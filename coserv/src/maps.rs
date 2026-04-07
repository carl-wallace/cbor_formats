//! Map-based structs from the CoSERV specification (draft-ietf-rats-coserv-05)

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

use cbor_derive::StructToMap;
use ciborium::tag::Required;
use cmw::arrays::{CborRecord, CborRecordCbor};
use common::*;
use corim::arrays::{
    AttestKeyTripleRecord, AttestKeyTripleRecordCbor, ConditionalEndorsementTripleRecord,
    ConditionalEndorsementTripleRecordCbor, EndorsedTripleRecord, EndorsedTripleRecordCbor,
    ReferenceTripleRecord, ReferenceTripleRecordCbor,
};
use corim::choices::{CryptoKeyTypeChoice, ProfileTypeChoice};
use cots::maps::{ConciseTaStoreMap, ConciseTaStoreMapCbor};
use serde::ser::Error as OtherError;

use crate::arrays::*;
use crate::choices::*;

// tdate = #6.0(tstr)

/// Text-based date/time (JSON-friendly form).
///
/// Corresponds to the CBOR `tdate` type (`#6.0(tstr)`) defined in RFC 8949.
pub type Tdate = String;

/// CBOR-encoded text-based date/time (`#6.0(tstr)`).
///
/// Use [Tdate] for the JSON-friendly form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum TdateCbor {
    T(Required<String, 0>),
}
impl TryFrom<&Value> for TdateCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tag(0, k) => match k.as_text() {
                Some(s) => Ok(Self::T(Required(s.to_string()))),
                None => Err("Expected text value inside tag 0 for TdateCbor".to_string()),
            },
            _ => Err("Failed to parse value as a TdateCbor".to_string()),
        }
    }
}
impl TryFrom<&TdateCbor> for Tdate {
    type Error = String;
    fn try_from(value: &TdateCbor) -> Result<Self, Self::Error> {
        match value {
            TdateCbor::T(Required(s)) => Ok(s.clone()),
        }
    }
}
impl TryFrom<&Tdate> for TdateCbor {
    type Error = String;
    fn try_from(value: &Tdate) -> Result<Self, Self::Error> {
        Ok(TdateCbor::T(Required(value.clone())))
    }
}

// coserv = {
//   profile: comid.oid-type / ~uri
//   query: query-map
//   ? results: results-map
// }

/// The top-level `coserv` map from [CoSERV Section 4.1].
///
/// ```text
/// coserv = {
///   profile: comid.oid-type / ~uri
///   query: query-map
///   ? results: results-map
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoservMap {
    #[cbor(tag = "0")]
    pub profile: ProfileTypeChoice,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub query: QueryMap,
    #[cbor(tag = "2", value = "Map", cbor = "true")]
    pub results: Option<ResultsMap>,
}

// query-map = {
//   artifact-type: artifact-type
//   environment-selector: environment-selector-map
//   result-type: result-type
// }

/// The `query-map` from [CoSERV Section 4.1].
///
/// ```text
/// query-map = {
///   artifact-type: artifact-type
///   environment-selector: environment-selector-map
///   result-type: result-type
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct QueryMap {
    #[cbor(tag = "0")]
    pub artifact_type: ArtifactType,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub environment_selector: EnvironmentSelectorMap,
    #[cbor(tag = "2")]
    pub result_type: ResultType,
}

// environment-selector-map = {
//   ? class: [+ stateful-class]
//   ? instance: [+ stateful-instance]
//   ? group: [+ stateful-group]
// }

/// The `environment-selector-map` from [CoSERV Section 4.1].
///
/// ```text
/// environment-selector-map = {
///   ? class: [+ stateful-class]
///   ? instance: [+ stateful-instance]
///   ? group: [+ stateful-group]
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EnvironmentSelectorMap {
    #[cbor(tag = "0", value = "Array", cbor = "true")]
    pub class: Option<Vec<StatefulClass>>,
    #[cbor(tag = "1", value = "Array", cbor = "true")]
    pub instance: Option<Vec<StatefulInstance>>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub group: Option<Vec<StatefulGroup>>,
}

// results-map = {
//   ? rvq: [+ refval-quad-map]
//   ? evq: [+ endval-quad-map]
//   ? ceq: [+ cond-endval-quad-map]
//   ? akq: [+ ak-quad-map]
//   ? tas: [+ cots-stmt-map]
//   expiry: tdate
//   ? source-artifacts: [+ cmw.cbor-record]
// }

/// The `results-map` from [CoSERV Section 4.1].
///
/// ```text
/// results-map = {
///   ? rvq: [+ refval-quad-map]
///   ? evq: [+ endval-quad-map]
///   ? ceq: [+ cond-endval-quad-map]
///   ? akq: [+ ak-quad-map]
///   ? tas: [+ cots-stmt-map]
///   expiry: tdate
///   ? source-artifacts: [+ cmw.cbor-record]
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ResultsMap {
    #[cbor(tag = "0", value = "Array", cbor = "true")]
    pub rvq: Option<Vec<RefvalQuadMap>>,
    #[cbor(tag = "1", value = "Array", cbor = "true")]
    pub evq: Option<Vec<EndvalQuadMap>>,
    #[cbor(tag = "2", value = "Array", cbor = "true")]
    pub ceq: Option<Vec<CondEndvalQuadMap>>,
    #[cbor(tag = "3", value = "Array", cbor = "true")]
    pub akq: Option<Vec<AkQuadMap>>,
    #[cbor(tag = "4", value = "Array", cbor = "true")]
    pub tas: Option<Vec<CotsStmtMap>>,
    #[cbor(tag = "10", cbor = "true")]
    pub expiry: Tdate,
    #[cbor(tag = "11", value = "Array", cbor = "true")]
    pub source_artifacts: Option<Vec<CborRecord>>,
}

// refval-quad-map = {
//   authorities: [+ $crypto-key-type-choice]
//   rv-triple: reference-triple-record
// }

/// The `refval-quad-map` from [CoSERV Section 4.1].
///
/// ```text
/// refval-quad-map = {
///   authorities: [+ $crypto-key-type-choice]
///   rv-triple: reference-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RefvalQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub rv_triple: ReferenceTripleRecord,
}

// endval-quad-map = {
//   authorities: [+ $crypto-key-type-choice]
//   ev-triple: endorsed-triple-record
// }

/// The `endval-quad-map` from [CoSERV Section 4.1].
///
/// ```text
/// endval-quad-map = {
///   authorities: [+ $crypto-key-type-choice]
///   ev-triple: endorsed-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EndvalQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub ev_triple: EndorsedTripleRecord,
}

// cond-endval-quad-map = {
//   authorities: [+ $crypto-key-type-choice]
//   ce-triple: conditional-endorsement-triple-record
// }

/// The `cond-endval-quad-map` from [CoSERV Section 4.1].
///
/// ```text
/// cond-endval-quad-map = {
///   authorities: [+ $crypto-key-type-choice]
///   ce-triple: conditional-endorsement-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CondEndvalQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub ce_triple: ConditionalEndorsementTripleRecord,
}

// ak-quad-map = {
//   authorities: [+ $crypto-key-type-choice]
//   ak-triple: attest-key-triple-record
// }

/// The `ak-quad-map` from [CoSERV Section 4.1].
///
/// ```text
/// ak-quad-map = {
///   authorities: [+ $crypto-key-type-choice]
///   ak-triple: attest-key-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AkQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub ak_triple: AttestKeyTripleRecord,
}

// cots-stmt-map = {
//   authorities: [+ $crypto-key-type-choice]
//   cots: concise-ta-store-map
// }

/// The `cots-stmt-map` from [CoSERV Section 4.1].
///
/// ```text
/// cots-stmt-map = {
///   authorities: [+ $crypto-key-type-choice]
///   cots: concise-ta-store-map
/// }
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CotsStmtMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", value = "Map", cbor = "true")]
    pub cots: ConciseTaStoreMap,
}
