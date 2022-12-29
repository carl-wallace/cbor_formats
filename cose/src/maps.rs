//! Map-based structs

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::{fmt, marker::PhantomData};

use ciborium::{cbor, value::Value};
use serde::{Deserialize, Deserializer, Serialize};
use serde::{
    __private::size_hint,
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

use cbor_derive::StructToMap;

use common::{TextOrInt, Tuple, TupleCbor};

// todo enforce presence of IV or Partial IV (but not both)
/// CBOR and JSON encoding/decoding of `Generic_Headers`, see [COSE Section 3].
///
/// ```text
/// Generic_Headers = (
///     ? 1 => int / tstr,  ; algorithm identifier
///     ? 2 => [+label],    ; criticality
///     ? 3 => tstr / int,  ; content type
///     ? 4 => bstr,        ; key identifier
///     ? ( 5 => bstr //    ; IV
///         6 => bstr )     ; Partial IV
/// )
/// header_map = {
///     Generic_Headers,
///     * label => values
/// }
/// ```
/// [COSE Section 3]: https://datatracker.ietf.org/doc/html/rfc9052#section-3
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct HeaderMap {
    #[cbor(tag = "1")]
    pub alg_id: Option<TextOrInt>,
    #[cbor(tag = "2", value = "Array")]
    pub criticality: Option<Vec<TextOrInt>>,
    #[cbor(tag = "3")]
    pub content_type: Option<TextOrInt>,
    #[cbor(tag = "4", value = "Bytes")]
    pub key_id: Option<Vec<u8>>,
    #[cbor(tag = "5", value = "Bytes")]
    pub iv: Option<Vec<u8>>,
    #[cbor(tag = "6", value = "Bytes")]
    pub partial_iv: Option<Vec<u8>>,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// CBOR and JSON encoding/decoding of `COSE_Key`, see [COSE Section 7].
///
/// ```text
/// COSE_Key = {
///     1 => tstr / int,          ; kty
///     ? 2 => bstr,              ; kid
///     ? 3 => tstr / int,        ; alg
///     ? 4 => [+ (tstr / int) ], ; key_ops
///     ? 5 => bstr,              ; Base IV
///     * label => values
/// }
/// ```
/// [COSE Section 7]: https://datatracker.ietf.org/doc/html/rfc9052#section-7
#[derive(Clone, Debug, PartialEq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseKey {
    #[cbor(tag = "1")]
    pub kty: Option<TextOrInt>,
    #[cbor(tag = "2", value = "Bytes")]
    pub kid: Option<Vec<u8>>,
    #[cbor(tag = "3")]
    pub alg: Option<TextOrInt>,
    #[cbor(tag = "4", value = "Array")]
    pub key_ops: Option<Vec<TextOrInt>>,
    #[cbor(tag = "5", value = "Bytes")]
    pub iv: Option<Vec<u8>>,
    #[cbor(value = "Array", cbor = "true")]
    pub other: Option<Vec<Tuple>>,
}

/// CBOR and JSON encoding/decoding of `COSE_KeySet`, see [COSE Section 7].
///
/// ```text
/// COSE_KeySet = [+COSE_Key]
/// ```
/// [COSE Section 7]: https://datatracker.ietf.org/doc/html/rfc9052#section-7
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CoseKeySet(pub Vec<CoseKey>);
