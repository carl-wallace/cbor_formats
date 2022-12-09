//! General-purpose array types

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use alloc::{vec, vec::Vec};

use cbor_derive::StructToArray;

/// The `hash-entry` type is defined in [CoRIM Section 1.3.8].
///
/// ```text
/// hash-entry = [
///    hash-alg-id: int
///    hash-value: bytes
///  ]
/// ```
///
/// [CoRIM Section 1.3.8]: https://datatracker.ietf.org/doc/html/draft-birkholz-rats-corim-03#section-1.3.8
#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HashEntry {
    #[cbor(value = "Integer")]
    pub hash_alg_id: u64,
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub hash_value: Vec<u8>,
}
