//! Map-based structs from the EAT Attestation Result (EAR) spec
//! ([draft-ietf-rats-ear-03]).
//!
//! | CDDL | Rust |
//! |------|------|
//! | `EAR` | [`Ear`] / [`EarCbor`] |
//! | `EAR-appraisal` | [`EarAppraisal`] / [`EarAppraisalCbor`] |
//! | `{ + text => EAR-appraisal }` | [`EarSubmods`] / [`EarSubmodsCbor`] |
//!
//! [draft-ietf-rats-ear-03]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ear-03

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData};

use ciborium::{cbor, value::Value};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

use ar4si::choices::TrustworthinessTier;
use ar4si::maps::{TrustworthinessVector, TrustworthinessVectorCbor, VerifierId, VerifierIdCbor};
use cbor_derive::StructToMap;
use common::GeneralProfile;
use common::tuple::Tuple;
#[allow(unused_imports)]
use common::tuple::TupleCbor;
use common::*;
#[allow(unused_imports)]
/// JSON encoding/decoding of `EAR`, see [EAR Section 4].
///
/// Use [`EarCbor`] for CBOR-encoded EARs.
///
/// ```text
/// EAR = {
///     eat.profile-label => "tag:ietf.org,2026:rats/ear#03"
///     ? status-label => ar4si.trustworthiness-tier
///     eat.iat-claim-label => ~eat.time-int
///     ? eat.exp-claim-label => ~eat.time-int
///     verifier-id-label => ar4si.verifier-id
///     ? raw-evidence-label => eat.binary-data
///     eat.submods-label => { + text => EAR-appraisal }
///     ? eat.nonce-label => eat.nonce-type
///     * $$ear-extension
/// }
///
/// raw-evidence-label = JC<"ear_raw_evidence", 1002>
/// verifier-id-label = JC<"ear_verifier_id", 1004>
///
/// ; EAT claim labels
/// profile-label = 265
/// iat-claim-label = 6
/// exp-claim-label = 4
/// submods-label = 266
/// nonce-label = 10
///
/// ; AR4SI claim labels
/// status-label = JC<"ear_status", 1000>
/// ```
///
/// [EAR Section 4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ear-03#section-4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Ear {
    #[cbor(tag = "265", value = "Text")]
    pub profile: String,
    #[cbor(tag = "1000")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrustworthinessTier>,
    #[cbor(tag = "6", value = "Integer")]
    pub iat: i64,
    #[cbor(tag = "4", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[cbor(tag = "1004", value = "Map", cbor = "true")]
    pub verifier_id: VerifierId,
    #[cbor(tag = "1002", value = "Bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_evidence: Option<Vec<u8>>,
    #[cbor(tag = "266", cbor = "true")]
    pub submods: EarSubmods,
    #[cbor(tag = "10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<NonceType>,
    #[cbor(value = "Array", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<Tuple>>,
}

/// JSON encoding/decoding of `EAR-appraisal`, see [EAR Section 4.1].
///
/// Use [`EarAppraisalCbor`] for CBOR-encoded tokens.
///
/// ```text
/// EAR-appraisal = {
///     ? eat.profile-label => eat.general-uri / eat.general-oid
///     status-label => ar4si.trustworthiness-tier
///     ? trustworthiness-vector-label => ar4si.trustworthiness-vector
///     ? appraisal-policy-ids-label => [ + text ]
///     ? eat.nonce-label => eat.nonce-type
///     * $$ear-appraisal-extension
/// }
///
/// status-label = JC<"ear_status", 1000>
/// trustworthiness-vector-label = JC<"ear_trustworthiness_vector", 1001>
/// appraisal-policy-ids-label = JC<"ear_appraisal_policy_ids", 1003>
/// ```
///
/// [EAR Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ear-03#section-4.1
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct EarAppraisal {
    #[cbor(tag = "265")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<GeneralProfile>,
    #[cbor(tag = "1000")]
    pub status: TrustworthinessTier,
    #[cbor(tag = "1001", value = "Map", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trustworthiness_vector: Option<TrustworthinessVector>,
    #[cbor(tag = "1003")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appraisal_policy_ids: Option<AppraisalPolicyIds>,
    #[cbor(tag = "10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<NonceType>,
    #[cbor(value = "Array", cbor = "true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<Tuple>>,
}

/// Wrapper for `[ + text ]` used in `appraisal-policy-ids-label`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppraisalPolicyIds(pub Vec<String>);

impl TryFrom<&Value> for AppraisalPolicyIds {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(a) => {
                let mut items = Vec::new();
                for v in a {
                    match v.as_text() {
                        Some(s) => items.push(s.to_string()),
                        None => {
                            return Err("AppraisalPolicyIds array element must be text".to_string());
                        }
                    }
                }
                if items.is_empty() {
                    return Err("AppraisalPolicyIds must contain at least one entry".to_string());
                }
                Ok(Self(items))
            }
            _ => Err("Failed to parse value as AppraisalPolicyIds".to_string()),
        }
    }
}

impl TryFrom<Value> for AppraisalPolicyIds {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// JSON encoding/decoding of the EAR submods map `{ + text => EAR-appraisal }`.
///
/// Use [`EarSubmodsCbor`] for CBOR-encoded EARs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EarSubmods(pub BTreeMap<String, EarAppraisal>);

/// CBOR encoding/decoding of the EAR submods map `{ + text => EAR-appraisal }`.
///
/// Use [`EarSubmods`] for JSON-encoded EARs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EarSubmodsCbor(pub BTreeMap<String, EarAppraisalCbor>);

impl TryFrom<&Value> for EarSubmodsCbor {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(entries) => {
                let mut map = BTreeMap::new();
                for (k, v) in entries {
                    let key = match k.as_text() {
                        Some(s) => s.to_string(),
                        None => {
                            return Err("EarSubmods map key must be a text string".to_string());
                        }
                    };
                    let appraisal = match v.as_map() {
                        Some(m) => EarAppraisalCbor::try_from(m.clone())?,
                        None => {
                            return Err(format!(
                                "EarSubmods map value for key '{}' must be a map",
                                key
                            ));
                        }
                    };
                    map.insert(key, appraisal);
                }
                if map.is_empty() {
                    return Err("EarSubmods must contain at least one entry".to_string());
                }
                Ok(Self(map))
            }
            _ => Err("Failed to parse value as EarSubmodsCbor".to_string()),
        }
    }
}

impl TryFrom<Value> for EarSubmodsCbor {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&EarSubmods> for EarSubmodsCbor {
    type Error = String;
    fn try_from(value: &EarSubmods) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::new();
        for (k, v) in &value.0 {
            let cbor: EarAppraisalCbor = v.try_into()?;
            map.insert(k.clone(), cbor);
        }
        Ok(Self(map))
    }
}

impl TryFrom<EarSubmods> for EarSubmodsCbor {
    type Error = String;
    fn try_from(value: EarSubmods) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&EarSubmodsCbor> for EarSubmods {
    type Error = String;
    fn try_from(value: &EarSubmodsCbor) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::new();
        for (k, v) in &value.0 {
            let appraisal: EarAppraisal = v.try_into()?;
            map.insert(k.clone(), appraisal);
        }
        Ok(Self(map))
    }
}

impl TryFrom<EarSubmodsCbor> for EarSubmods {
    type Error = String;
    fn try_from(value: EarSubmodsCbor) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
