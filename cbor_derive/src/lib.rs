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

/// The StructToMap derive macro marshals data from a structure into a Vec<(Value, Value)> for use with
/// the Ciborium library. For each structure the following artifacts are generated:
/// - an alternative structure named with Cbor appended to the original structure name
/// - Serialize and Deserialize implementations for the alternative structure
/// - TryFrom implementations to move between alternative structure and original structure
/// - TryFrom implementations to move between alternative structure and Vec<(Value, Value)>
///
/// The following values are used from the cbor field attribute:
/// - `tag`: indicates the integer key used to identify the associated field. The value will be
/// included as the first element in a (Value, Value) production
/// - `value`: indicates the type of ciborium Value used to represent the field. This is omitted if
/// a Value is not used to represent the field.
/// - `cbor`: indicates the inner type structure should have a Cbor suffix appended when generating
/// or parsing CBOR-encoded representation of the field.
#[proc_macro_derive(StructToMap, attributes(cbor))]
#[proc_macro_error]
pub fn derive_struct_to_map(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DeriveStructToMap::new(input).to_tokens().into()
}

/// The StructToArray derive macro marshals data from a structure into a Vec<Value> for use with
/// the Ciborium library. For each structure the following artifacts are generated:
/// - an alternative structure named with Cbor appended to the original structure name
/// - Serialize and Deserialize implementations for the alternative structure
/// - TryFrom implementations to move between alternative structure and original structure
/// - TryFrom implementations to move between alternative structure and Vec<Value>
///
/// The following values are used from the cbor field attribute:
/// - `value`: indicates the type of ciborium Value used to represent the field. This is omitted if
/// a Value is not used to represent the field.
/// - `cbor`: indicates the inner type structure should have a Cbor suffix appended when generating
/// or parsing CBOR-encoded representation of the field.
#[proc_macro_derive(StructToArray, attributes(cbor))]
#[proc_macro_error]
pub fn derive_struct_to_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DeriveStructToArray::new(input).to_tokens().into()
}

/// The StructToOneOrMore macro marshals data to/from an enum that implements the one-or-more CDDL
/// mechanism. An enum named OneOrMore<name>Cbor is emitted for associated structures along with a
/// set of TryFrom implementations.
#[proc_macro_derive(StructToOneOrMore, attributes(cbor))]
#[proc_macro_error]
pub fn derive_struct_to_one_or_more(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    DeriveStructToOneOrMore::new(input).to_tokens().into()
}
