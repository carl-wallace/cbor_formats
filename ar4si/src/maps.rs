//! Map-based structs from the Attestation Results for Secure Interactions spec
//! ([draft-ietf-rats-ar4si-09]).
//!
//! | CDDL | Rust |
//! |------|------|
//! | `verifier-id` | [`VerifierId`] / [`VerifierIdCbor`] |
//! | `trustworthiness-vector` | [`TrustworthinessVector`] / [`TrustworthinessVectorCbor`] |
//!
//! [draft-ietf-rats-ar4si-09]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData};

use cbor_derive::StructToMap;
use ciborium::{cbor, value::Value};
#[allow(unused_imports)]
use common::tuple::TupleCbor;
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

/// JSON encoding/decoding of `verifier-id`, see [AR4SI Section 2.4].
///
/// Use [`VerifierIdCbor`] for CBOR-encoded tokens.
///
/// ```text
/// verifier-id = {
///     developer-label => text
///     build-label => text
/// }
///
/// developer-label = JC<"developer", 0>
/// build-label = JC<"build", 1>
/// ```
///
/// [AR4SI Section 2.4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09#section-2.4
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct VerifierId {
    #[cbor(tag = "0", value = "Text")]
    pub developer: String,
    #[cbor(tag = "1", value = "Text")]
    pub build: String,
}

/// JSON encoding/decoding of `trustworthiness-vector`, see [AR4SI Section 2.3.2].
///
/// Use [`TrustworthinessVectorCbor`] for CBOR-encoded tokens.
///
/// ```text
/// trustworthiness-vector = non-empty<{
///     ? instance-identity-label => trustworthiness-claim
///     ? configuration-label => trustworthiness-claim
///     ? executables-label => trustworthiness-claim
///     ? file-system-label => trustworthiness-claim
///     ? hardware-label => trustworthiness-claim
///     ? runtime-opaque-label => trustworthiness-claim
///     ? storage-opaque-label => trustworthiness-claim
///     ? sourced-data-label => trustworthiness-claim
/// }>
///
/// instance-identity-label = JC<"instance-identity", 0>
/// configuration-label = JC<"configuration", 1>
/// executables-label = JC<"executables", 2>
/// file-system-label = JC<"file-system", 3>
/// hardware-label = JC<"hardware", 4>
/// runtime-opaque-label = JC<"runtime-opaque", 5>
/// storage-opaque-label = JC<"storage-opaque", 6>
/// sourced-data-label = JC<"sourced-data", 7>
///
/// trustworthiness-claim = -128..127
/// ```
///
/// [AR4SI Section 2.3.2]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09#section-2.3.2
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[cbor(non_empty = "true")]
#[allow(missing_docs)]
pub struct TrustworthinessVector {
    #[cbor(tag = "0", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity: Option<i8>,
    #[cbor(tag = "1", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<i8>,
    #[cbor(tag = "2", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executables: Option<i8>,
    #[cbor(tag = "3", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<i8>,
    #[cbor(tag = "4", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<i8>,
    #[cbor(tag = "5", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_opaque: Option<i8>,
    #[cbor(tag = "6", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_opaque: Option<i8>,
    #[cbor(tag = "7", value = "Integer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourced_data: Option<i8>,
}
