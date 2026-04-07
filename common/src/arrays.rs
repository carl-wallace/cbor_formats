//! General-purpose array types

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{de::Error, de::Visitor};
use serde::{Deserialize, Serialize};

use alloc::{vec, vec::Vec};

use alloc::format;
use alloc::string::{String, ToString};
use cbor_derive::StructToArray;

/// The `hash-entry` type is defined in [CoRIM Section 7.7].
///
/// ```text
/// hash-entry = [
///    hash-alg-id: int
///    hash-value: bytes
///  ]
/// ```
///
/// [CoRIM Section 7.7]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.7
#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HashEntry {
    #[cbor(value = "Integer")]
    pub hash_alg_id: u64,
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub hash_value: Vec<u8>,
}

/// The `masked-raw-value` type is defined in [CoRIM Section 5.1.4.5.6].
///
/// ```text
/// masked-raw-value = [raw-value: bytes, mask: bytes]
/// tagged-masked-raw-value = #6.563(masked-raw-value)
/// ```
///
/// [CoRIM Section 5.1.4.5.6]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.5.6
#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MaskedRawValue {
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub mask: Vec<u8>,
}

/// The `int-range` type is defined in [CoRIM Section 5.1.4.8].
///
/// ```text
/// int-range = [min: int, max: int]
/// tagged-int-range = #6.564(int-range)
/// ```
///
/// [CoRIM Section 5.1.4.8]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4.8
#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct IntRange {
    #[cbor(value = "Integer")]
    pub min: i64,
    #[cbor(value = "Integer")]
    pub max: i64,
}
