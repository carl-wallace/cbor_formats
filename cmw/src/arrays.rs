//! Array-based structs from the CMW specification (draft-ietf-rats-msg-wrap-23)

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{de::Error, de::Visitor};

use alloc::{vec, vec::Vec};

use alloc::format;
use alloc::string::{String, ToString};
use cbor_derive::StructToArray;

use crate::choices::*;

// cbor-record = [
//     type: coap-content-format
//     value: bstr
//     ? ind: uint .bits cm-type
// ]

/// The `cbor-record` type is defined in [CMW Section 4].
///
/// ```text
/// cbor-record = [
///     type: coap-content-format
///     value: bstr
///     ? ind: uint .bits cm-type
/// ]
/// ```
///
/// [CMW Section 4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-4
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CborRecord {
    pub record_type: CoapContentFormatOrMediaType,
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,
    pub ind: Option<CmwIndicator>,
}

// json-record = [
//     type: media-type
//     value: tstr
//     ? ind: uint .bits cm-type
// ]

/// The `json-record` type is defined in [CMW Section 4].
///
/// ```text
/// json-record = [
///     type: media-type
///     value: tstr
///     ? ind: uint .bits cm-type
/// ]
/// ```
///
/// [CMW Section 4]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-4
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct JsonRecord {
    #[cbor(value = "Text")]
    pub media_type: String,
    #[cbor(value = "Text")]
    pub value: String,
    pub ind: Option<CmwIndicator>,
}
