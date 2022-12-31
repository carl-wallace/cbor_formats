#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

use proc_macro2::Span;
use quote::quote;
use syn::Lifetime;

/// Get the default lifetime.
fn default_lifetime() -> proc_macro2::TokenStream {
    let lifetime = Lifetime::new("'__der_lifetime", Span::call_site());
    quote!(#lifetime)
}

mod attributes;
mod cbor_derive_utils;
mod field;
mod struct_to_array;
mod struct_to_map;
mod struct_to_one_or_more;

use crate::struct_to_array::DeriveStructToArray;
use crate::struct_to_map::DeriveStructToMap;
use crate::struct_to_one_or_more::DeriveStructToOneOrMore;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

/// The `StructToMap` derive macro marshals data from a structure into a `Vec<(Value, Value)>` for use with
/// the [ciborium](https://crates.io/crates/ciborium) library. For each structure the following artifacts are generated:
/// - an alternative structure named with `Cbor` appended to the original structure name
/// - `Serialize` and `Deserialize` implementations for the alternative structure
/// - `TryFrom` implementations to move between alternative structure and original structure
/// - `TryFrom` implementations to move between alternative structure and `Vec<(Value, Value)>`
///
/// The following values are used from the `cbor` field attribute:
/// - `tag`: indicates the integer key used to identify the associated field. The value will be
/// included as the first element in a `(Value, Value)` production
/// - `value`: indicates the type of ciborium `Value` used to represent the field. This is omitted if
/// a `Value` is not used to represent the field.
/// - `cbor`: indicates the inner type structure should have a `Cbor` suffix appended when generating
/// or parsing CBOR-encoded representation of the field.
///
/// See the [cbor_derive](index.html#example) crate for a `StructToMap` example.
#[proc_macro_derive(StructToMap, attributes(cbor))]
#[proc_macro_error]
pub fn derive_struct_to_map(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DeriveStructToMap::new(input).to_tokens().into()
}

/// The `StructToArray` derive macro marshals data from a structure into a `Vec<Value>` for use with
/// the [ciborium](https://crates.io/crates/ciborium) library. For each structure the following artifacts are generated:
/// - an alternative structure named with `Cbor` appended to the original structure name
/// - `Serialize` and `Deserialize` implementations for the alternative structure
/// - `TryFrom` implementations to move between alternative structure and original structure
/// - `TryFrom` implementations to move between alternative structure and `Vec<Value>`
///
/// The following values are used from the `cbor` field attribute:
/// - `value`: indicates the type of ciborium `Value` used to represent the field. This is omitted if
/// a `Value` is not used to represent the field.
/// - `cbor`: indicates the inner type structure should have a `Cbor` suffix appended when generating
/// or parsing CBOR-encoded representation of the field.
///
/// The hash-entry array is defined in [CoRIM Section 1.3.8]:
///
/// ```text
/// hash-entry = [
///    hash-alg-id: int
///    hash-value: bytes
///  ]
/// ```
///
/// The [HashEntry](../common/HashEntry) struct uses the `StructToArray` procedural macro and is defined
/// in the [common crate](../common/index.html) in this workspace.
///
/// ```rust
/// use core::fmt;
///
/// use ciborium::{cbor, value::Value};
/// use serde::{Serialize, Deserialize};
/// use serde::__private::{PhantomData, size_hint};
/// use serde::de::{Visitor, Error as OtherError};
/// use serde::ser::Error;
///
/// use cbor_derive::StructToArray;
///
/// #[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
/// #[allow(missing_docs)]
/// pub struct HashEntry {
///     #[cbor(value = "Integer")]
///     pub hash_alg_id: u64,
///     #[cbor(value = "Bytes")]
///     #[serde(with = "serde_bytes")]
///     pub hash_value: Vec<u8>,
/// }
/// ```
/// The code below shows use of the [HashEntry](../common/arrays/struct.HashEntry.html) struct for
/// JSON encoding/decoding and the [HashEntryCbor](../common/arrays/struct.HashEntryCbor.html) struct
/// for CBOR encoding/decoding.
///
/// ```rust
/// use ciborium::ser::into_writer;
/// use ciborium::de::from_reader;
/// use hex_literal::hex;
///
/// use common::arrays::{HashEntry, HashEntryCbor};

///
/// let some_bytes = hex!("a200c11a637cffdc01c11a637d0decffa200c11a637cffdc01c11a637d0decff");
/// let scratch = HashEntryCbor {
///     hash_alg_id: 1,
///     hash_value: some_bytes.to_vec(),
/// };
///
/// // CBOR encode and decode
/// let mut encoded_cbor = vec![];
/// let _ = into_writer(&scratch, &mut encoded_cbor);
/// let decoded: HashEntryCbor = from_reader(encoded_cbor.clone().as_slice()).unwrap();
/// assert_eq!(decoded, scratch);
///
/// // Convert CBOR struct to JSON struct then JSON encode and decode
/// let json_from_cbor: HashEntry = decoded.try_into().unwrap();
/// let encoded_json = serde_json::to_string(&json_from_cbor).unwrap();
/// let decoded_json: HashEntry = serde_json::from_str(encoded_json.as_str()).unwrap();
/// assert_eq!(decoded_json, json_from_cbor);
///
/// // Convert JSON struct to CBOR struct
/// let roundtrip: HashEntryCbor = decoded_json.try_into().unwrap();
/// assert_eq!(roundtrip, scratch);
/// ```
#[proc_macro_derive(StructToArray, attributes(cbor))]
#[proc_macro_error]
pub fn derive_struct_to_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DeriveStructToArray::new(input).to_tokens().into()
}

/// The `StructToOneOrMore` macro marshals data to/from an enum that implements the one-or-more CDDL
/// mechanism. An enum named `OneOrMore<name>Cbor` is emitted for associated structures along with a
/// set of `TryFrom` implementations.
#[proc_macro_derive(StructToOneOrMore, attributes(cbor))]
#[proc_macro_error]
pub fn derive_struct_to_one_or_more(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DeriveStructToOneOrMore::new(input).to_tokens().into()
}
