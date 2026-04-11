//! Map-based structs from the CoSERV specification ([draft-ietf-rats-coserv-05]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `coserv` | [`CoservMap`] / [`CoservMapCbor`] |
//! | `query` | [`QueryMap`] / [`QueryMapCbor`] |
//! | `environment-selector-map` | [`EnvironmentSelectorMap`] / [`EnvironmentSelectorMapCbor`] |
//! | `results` | [`ResultsMap`] / [`ResultsMapCbor`] |
//! | `refval-quad` | [`RefvalQuadMap`] / [`RefvalQuadMapCbor`] |
//! | `endval-quad` | [`EndvalQuadMap`] / [`EndvalQuadMapCbor`] |
//! | `cond-endval-quad` | [`CondEndvalQuadMap`] / [`CondEndvalQuadMapCbor`] |
//! | `ak-quad` | [`AkQuadMap`] / [`AkQuadMapCbor`] |
//! | `cots-stmt` | [`CotsStmtMap`] / [`CotsStmtMapCbor`] |
//! | `tdate` | [`Tdate`] / [`TdateCbor`] |
//!
//! [draft-ietf-rats-coserv-05]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05

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
use corim::choices::CryptoKeyTypeChoice;
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
//   query: query
//   ? results: results
// }

/// The top-level `coserv` map from [CoSERV Section 4].
///
/// ```text
/// coserv = {
///   profile: comid.oid-type / ~uri
///   query: query
///   ? results: results
/// }
/// ```
///
/// [CoSERV Section 4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoservMap {
    #[cbor(tag = "0")]
    pub profile: CoservProfile,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub query: QueryMap,
    #[cbor(tag = "2", value = "Map", cbor = "true")]
    pub results: Option<ResultsMap>,
}

// query = {
//   artifact-type: artifact-type
//   environment-selector: environment-selector-map
//   result-type: result-type
// }

/// The `query` from [CoSERV Section 4.3].
///
/// ```text
/// query = {
///   artifact-type: artifact-type
///   environment-selector: environment-selector-map
///   result-type: result-type
/// }
/// ```
///
/// [CoSERV Section 4.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.3
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

// environment-selector-map = { selector }
//
// selector //= ( &(class: 0) => [+ stateful-class] )
// selector //= ( &(instance: 1) => [+ stateful-instance] )
// selector //= ( &(group: 2) => [+ stateful-group] )

/// The `environment-selector-map` from [CoSERV Section 4.3.2].
///
/// ```text
/// environment-selector-map = { selector }
///
/// selector //= ( &(class: 0) => [+ stateful-class] )
/// selector //= ( &(instance: 1) => [+ stateful-instance] )
/// selector //= ( &(group: 2) => [+ stateful-group] )
/// ```
///
/// The three selector types are mutually exclusive per the spec: "these three
/// environment definitions are mutually-exclusive". The struct uses `Option`
/// fields for decode flexibility; call [`validate`](Self::validate) to enforce
/// mutual exclusivity and the `[+ ...]` non-empty constraint.
///
/// [CoSERV Section 4.3.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.3.2
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

impl EnvironmentSelectorMap {
    /// Validates that exactly one selector type is present and that its array
    /// is non-empty, per the CDDL `selector` group choice with `[+ ...]`.
    pub fn validate(&self) -> Result<(), String> {
        let count =
            self.class.is_some() as u8 + self.instance.is_some() as u8 + self.group.is_some() as u8;
        if count == 0 {
            return Err("environment-selector-map must contain exactly one selector".to_string());
        }
        if count > 1 {
            return Err(
                "environment-selector-map must contain exactly one selector type, not a mixture"
                    .to_string(),
            );
        }
        if self.class.as_ref().is_some_and(|v| v.is_empty())
            || self.instance.as_ref().is_some_and(|v| v.is_empty())
            || self.group.as_ref().is_some_and(|v| v.is_empty())
        {
            return Err("selector array must be non-empty".to_string());
        }
        Ok(())
    }
}

// results = {
//   result-set
//   &(expiry: 10) => tdate
//   ? &(source-artifacts: 11) => [+ cmw.cbor-record]
// }
//
// result-set //= reference-values
// result-set //= endorsed-values
// result-set //= trust-anchors
//
// reference-values = ( &(rvq: 0) => [* refval-quad] )
// endorsed-values  = ( &(evq: 1) => [* endval-quad],  &(ceq: 2) => [* cond-endval-quad] )
// trust-anchors    = ( &(akq: 3) => [* ak-quad],      &(tas: 4) => [* cots-stmt] )

/// The `result-set` group choice from [CoSERV Section 4.4].
///
/// ```text
/// result-set //= reference-values
/// result-set //= endorsed-values
/// result-set //= trust-anchors
///
/// reference-values = ( &(rvq: 0) => [* refval-quad] )
/// endorsed-values  = ( &(evq: 1) => [* endval-quad],  &(ceq: 2) => [* cond-endval-quad] )
/// trust-anchors    = ( &(akq: 3) => [* ak-quad],      &(tas: 4) => [* cots-stmt] )
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum ResultSet {
    ReferenceValues {
        rvq: Vec<RefvalQuadMap>,
    },
    EndorsedValues {
        evq: Vec<EndvalQuadMap>,
        ceq: Vec<CondEndvalQuadMap>,
    },
    TrustAnchors {
        akq: Vec<AkQuadMap>,
        tas: Vec<CotsStmtMap>,
    },
}

/// CBOR-encoded form of [`ResultSet`].
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum ResultSetCbor {
    ReferenceValues {
        rvq: Vec<RefvalQuadMapCbor>,
    },
    EndorsedValues {
        evq: Vec<EndvalQuadMapCbor>,
        ceq: Vec<CondEndvalQuadMapCbor>,
    },
    TrustAnchors {
        akq: Vec<AkQuadMapCbor>,
        tas: Vec<CotsStmtMapCbor>,
    },
}

impl TryFrom<&ResultSetCbor> for ResultSet {
    type Error = String;
    fn try_from(value: &ResultSetCbor) -> Result<Self, Self::Error> {
        match value {
            ResultSetCbor::ReferenceValues { rvq } => Ok(ResultSet::ReferenceValues {
                rvq: rvq
                    .iter()
                    .map(RefvalQuadMap::try_from)
                    .collect::<Result<_, _>>()?,
            }),
            ResultSetCbor::EndorsedValues { evq, ceq } => Ok(ResultSet::EndorsedValues {
                evq: evq
                    .iter()
                    .map(EndvalQuadMap::try_from)
                    .collect::<Result<_, _>>()?,
                ceq: ceq
                    .iter()
                    .map(CondEndvalQuadMap::try_from)
                    .collect::<Result<_, _>>()?,
            }),
            ResultSetCbor::TrustAnchors { akq, tas } => Ok(ResultSet::TrustAnchors {
                akq: akq
                    .iter()
                    .map(AkQuadMap::try_from)
                    .collect::<Result<_, _>>()?,
                tas: tas
                    .iter()
                    .map(CotsStmtMap::try_from)
                    .collect::<Result<_, _>>()?,
            }),
        }
    }
}

impl TryFrom<&ResultSet> for ResultSetCbor {
    type Error = String;
    fn try_from(value: &ResultSet) -> Result<Self, Self::Error> {
        match value {
            ResultSet::ReferenceValues { rvq } => Ok(ResultSetCbor::ReferenceValues {
                rvq: rvq
                    .iter()
                    .map(RefvalQuadMapCbor::try_from)
                    .collect::<Result<_, _>>()?,
            }),
            ResultSet::EndorsedValues { evq, ceq } => Ok(ResultSetCbor::EndorsedValues {
                evq: evq
                    .iter()
                    .map(EndvalQuadMapCbor::try_from)
                    .collect::<Result<_, _>>()?,
                ceq: ceq
                    .iter()
                    .map(CondEndvalQuadMapCbor::try_from)
                    .collect::<Result<_, _>>()?,
            }),
            ResultSet::TrustAnchors { akq, tas } => Ok(ResultSetCbor::TrustAnchors {
                akq: akq
                    .iter()
                    .map(AkQuadMapCbor::try_from)
                    .collect::<Result<_, _>>()?,
                tas: tas
                    .iter()
                    .map(CotsStmtMapCbor::try_from)
                    .collect::<Result<_, _>>()?,
            }),
        }
    }
}

/// The `results` from [CoSERV Section 4.4].
///
/// ```text
/// results = {
///   result-set
///   &(expiry: 10) => tdate
///   ? &(source-artifacts: 11) => [+ cmw.cbor-record]
/// }
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ResultsMap {
    pub result_set: ResultSet,
    pub expiry: Tdate,
    pub source_artifacts: Option<Vec<CborRecord>>,
}

/// CBOR-encoded form of [`ResultsMap`].
///
/// The `result-set` group is flattened into the map alongside `expiry` and
/// `source-artifacts` on the wire.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ResultsMapCbor {
    pub result_set: ResultSetCbor,
    pub expiry: TdateCbor,
    pub source_artifacts: Option<Vec<CborRecordCbor>>,
}

impl TryFrom<&ResultsMapCbor> for ResultsMap {
    type Error = String;
    fn try_from(value: &ResultsMapCbor) -> Result<Self, Self::Error> {
        Ok(ResultsMap {
            result_set: ResultSet::try_from(&value.result_set)?,
            expiry: Tdate::try_from(&value.expiry)?,
            source_artifacts: match &value.source_artifacts {
                Some(v) => Some(
                    v.iter()
                        .map(CborRecord::try_from)
                        .collect::<Result<_, _>>()?,
                ),
                None => None,
            },
        })
    }
}

impl TryFrom<ResultsMapCbor> for ResultsMap {
    type Error = String;
    fn try_from(value: ResultsMapCbor) -> Result<Self, Self::Error> {
        ResultsMap::try_from(&value)
    }
}

impl TryFrom<&ResultsMap> for ResultsMapCbor {
    type Error = String;
    fn try_from(value: &ResultsMap) -> Result<Self, Self::Error> {
        Ok(ResultsMapCbor {
            result_set: ResultSetCbor::try_from(&value.result_set)?,
            expiry: TdateCbor::try_from(&value.expiry)?,
            source_artifacts: match &value.source_artifacts {
                Some(v) => Some(
                    v.iter()
                        .map(CborRecordCbor::try_from)
                        .collect::<Result<_, _>>()?,
                ),
                None => None,
            },
        })
    }
}

impl TryFrom<Vec<(Value, Value)>> for ResultsMapCbor {
    type Error = String;
    fn try_from(value: Vec<(Value, Value)>) -> Result<Self, Self::Error> {
        let mut m: BTreeMap<i32, Value> = BTreeMap::new();
        for (k, v) in value {
            let index: i32 = k
                .as_integer()
                .and_then(|i| i.try_into().ok())
                .ok_or_else(|| "Expected integer key in results map".to_string())?;
            m.insert(index, v);
        }

        let expiry = m
            .get(&10)
            .ok_or_else(|| "Missing required expiry (label 10)".to_string())
            .and_then(TdateCbor::try_from)?;

        let source_artifacts = match m.get(&11) {
            Some(v) => match v.as_array() {
                Some(a) => Some(
                    a.iter()
                        .map(|v| CborRecordCbor::try_from(v.clone()))
                        .collect::<Result<_, _>>()?,
                ),
                None => return Err("source-artifacts (label 11) must be an array".to_string()),
            },
            None => None,
        };

        // Determine which result-set variant is present based on labels
        let has_rvq = m.contains_key(&0);
        let has_evq = m.contains_key(&1);
        let has_ceq = m.contains_key(&2);
        let has_akq = m.contains_key(&3);
        let has_tas = m.contains_key(&4);

        let result_set = if has_rvq && !has_evq && !has_ceq && !has_akq && !has_tas {
            let rvq = parse_quad_array(&m, 0, "rvq")?;
            ResultSetCbor::ReferenceValues { rvq }
        } else if has_evq && has_ceq && !has_rvq && !has_akq && !has_tas {
            let evq = parse_quad_array(&m, 1, "evq")?;
            let ceq = parse_quad_array(&m, 2, "ceq")?;
            ResultSetCbor::EndorsedValues { evq, ceq }
        } else if has_akq && has_tas && !has_rvq && !has_evq && !has_ceq {
            let akq = parse_quad_array(&m, 3, "akq")?;
            let tas = parse_quad_array(&m, 4, "tas")?;
            ResultSetCbor::TrustAnchors { akq, tas }
        } else {
            return Err(
                "Invalid result-set: must be exactly one of reference-values (0), \
                 endorsed-values (1,2), or trust-anchors (3,4)"
                    .to_string(),
            );
        };

        Ok(ResultsMapCbor {
            result_set,
            expiry,
            source_artifacts,
        })
    }
}

/// Helper to parse an array of CBOR map values from a label in the results map.
fn parse_quad_array<T>(m: &BTreeMap<i32, Value>, label: i32, name: &str) -> Result<Vec<T>, String>
where
    T: TryFrom<Value, Error = String>,
{
    match m.get(&label) {
        Some(v) => match v.as_array() {
            Some(a) => a.iter().map(|v| T::try_from(v.clone())).collect(),
            None => Err(format!("{name} (label {label}) must be an array")),
        },
        None => Ok(Vec::new()),
    }
}

impl TryFrom<&ResultsMapCbor> for Vec<(Value, Value)> {
    type Error = String;
    fn try_from(value: &ResultsMapCbor) -> Result<Self, Self::Error> {
        let mut v: Vec<(Value, Value)> = Vec::new();

        // Serialize the result-set fields
        match &value.result_set {
            ResultSetCbor::ReferenceValues { rvq } => {
                v.push((
                    cbor!(0).map_err(|e| format!("{e:?}"))?,
                    Value::serialized(rvq).map_err(|e| format!("{e:?}"))?,
                ));
            }
            ResultSetCbor::EndorsedValues { evq, ceq } => {
                v.push((
                    cbor!(1).map_err(|e| format!("{e:?}"))?,
                    Value::serialized(evq).map_err(|e| format!("{e:?}"))?,
                ));
                v.push((
                    cbor!(2).map_err(|e| format!("{e:?}"))?,
                    Value::serialized(ceq).map_err(|e| format!("{e:?}"))?,
                ));
            }
            ResultSetCbor::TrustAnchors { akq, tas } => {
                v.push((
                    cbor!(3).map_err(|e| format!("{e:?}"))?,
                    Value::serialized(akq).map_err(|e| format!("{e:?}"))?,
                ));
                v.push((
                    cbor!(4).map_err(|e| format!("{e:?}"))?,
                    Value::serialized(tas).map_err(|e| format!("{e:?}"))?,
                ));
            }
        }

        // expiry (label 10)
        v.push((
            cbor!(10).map_err(|e| format!("{e:?}"))?,
            Value::serialized(&value.expiry).map_err(|e| format!("{e:?}"))?,
        ));

        // source-artifacts (label 11, optional)
        if let Some(sa) = &value.source_artifacts {
            v.push((
                cbor!(11).map_err(|e| format!("{e:?}"))?,
                Value::serialized(sa).map_err(|e| format!("{e:?}"))?,
            ));
        }

        Ok(v)
    }
}

impl Serialize for ResultsMapCbor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v: Vec<(Value, Value)> = self.try_into().map_err(S::Error::custom)?;
        Value::Map(v).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ResultsMapCbor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor;
        impl<'de> Visitor<'de> for MapVisitor {
            type Value = Vec<(Value, Value)>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = Vec::with_capacity(map.size_hint().unwrap_or(0).min(4096));
                while let Some(value) = map.next_entry()? {
                    values.push(value);
                }
                values.retain(|(_, v)| *v != Value::Null);
                Ok(values)
            }
        }
        let pairs = deserializer.deserialize_map(MapVisitor)?;
        ResultsMapCbor::try_from(pairs).map_err(D::Error::custom)
    }
}

// refval-quad = {
//   authorities: [+ $crypto-key-type-choice]
//   rv-triple: reference-triple-record
// }

/// The `refval-quad` from [CoSERV Section 4.4].
///
/// ```text
/// refval-quad = {
///   authorities: [+ $crypto-key-type-choice]
///   rv-triple: reference-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RefvalQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub rv_triple: ReferenceTripleRecord,
}

// endval-quad = {
//   authorities: [+ $crypto-key-type-choice]
//   ev-triple: endorsed-triple-record
// }

/// The `endval-quad` from [CoSERV Section 4.4].
///
/// ```text
/// endval-quad = {
///   authorities: [+ $crypto-key-type-choice]
///   ev-triple: endorsed-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EndvalQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub ev_triple: EndorsedTripleRecord,
}

// cond-endval-quad = {
//   authorities: [+ $crypto-key-type-choice]
//   ce-triple: conditional-endorsement-triple-record
// }

/// The `cond-endval-quad` from [CoSERV Section 4.4].
///
/// ```text
/// cond-endval-quad = {
///   authorities: [+ $crypto-key-type-choice]
///   ce-triple: conditional-endorsement-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CondEndvalQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub ce_triple: ConditionalEndorsementTripleRecord,
}

// ak-quad = {
//   authorities: [+ $crypto-key-type-choice]
//   ak-triple: attest-key-triple-record
// }

/// The `ak-quad` from [CoSERV Section 4.4].
///
/// ```text
/// ak-quad = {
///   authorities: [+ $crypto-key-type-choice]
///   ak-triple: attest-key-triple-record
/// }
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AkQuadMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", cbor = "true")]
    pub ak_triple: AttestKeyTripleRecord,
}

// cots-stmt = {
//   authorities: [+ $crypto-key-type-choice]
//   cots: concise-ta-store-map
// }

/// The `cots-stmt` from [CoSERV Section 4.4].
///
/// ```text
/// cots-stmt = {
///   authorities: [+ $crypto-key-type-choice]
///   cots: concise-ta-store-map
/// }
/// ```
///
/// [CoSERV Section 4.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CotsStmtMap {
    #[cbor(tag = "1", value = "Array")]
    pub authorities: Vec<CryptoKeyTypeChoice>,
    #[cbor(tag = "2", value = "Map", cbor = "true")]
    pub cots: ConciseTaStoreMap,
}
