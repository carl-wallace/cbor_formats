#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// nonce-type = bstr .size (8..64)
//
// ueid-type = bstr .size (7..33)
//
// oemid-pen = int
//
// oemid-ieee = oemid-ieee-cbor
// oemid-ieee-cbor = bstr .size 3
// oemid-ieee-json = base64-url-text .size 4
//
// oemid-random = oemid-random-cbor
// oemid-random-cbor = bstr .size 16
// oemid-random-json = base64-url-text .size 24
//
// hardware-model-type = bytes .size (1..32)
//
// spdx-json = text
//
// cyclone-dx-json = text
// cyclone-dx-xml  = text
//
// suit-directive-process-dependency = 19
//
//
// BUNDLE-Tagged-Message   = #6.602(BUNDLE-Untagged-Message)
// BUNDLE-Untagged-Message = Detached-EAT-Bundle
//
//
//
// json-wrapped-claims-set = base64-url-text
//
// cbor-wrapped-claims-set = bstr .cbor Claims-Set
//
// string-or-uri = text
//
// oid = #6.111(bstr)
// roid = #6.110(bstr)
// pen = #6.112(bstr)
//
//
// ; The payload of the COSE_Message is always a Claims-Set
//
// ; The contents of a CWT Tag must always be a COSE tag
// ;CWT-Tagged-Message = #6.61(COSE_Tagged_Message)
//
// ; An untagged CWT may be a COSE tag or not
//
//
// ;JWT-Message =
// ;   text .regexp "[A-Za-z0-9_=-]+\.[A-Za-z0-9_=-]+\.[A-Za-z0-9_=-]+"
//
// ;$EAT-CBOR-Tagged-Token /= #6.601(Claims-Set)
// ;$EAT-CBOR-Untagged-Token /= Claims-Set
pub mod arrays;
mod cbor_specific;
pub mod choices;
mod json_specific;
pub mod maps;
