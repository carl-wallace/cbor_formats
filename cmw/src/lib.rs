#![forbid(unsafe_code)]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::mod_module_files,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//!
//! ## CDDL-to-Rust Type Mapping
//!
//! The following table maps CDDL productions from
//! [draft-ietf-rats-msg-wrap-23 Section 3](https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3)
//! to their Rust implementations, organized by specification section.
//!
//! ### CMW Top-Level ([Section 3](https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `cmw = json-cmw / cbor-cmw` | [`choices::Cmw`] |
//! | `cbor-cmw = cbor-record / cbor-collection / $cbor-tag` | [`choices::CborCmw`] |
//! | `json-cmw = json-record / json-collection` | [`choices::JsonCmw`] |
//!
//! ### Records ([Section 3.1](https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3.1))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `cbor-record = [type, value, ? ind]` | [`arrays::CborRecord`] / [`arrays::CborRecordCbor`] |
//! | `json-record = [type, value, ? ind]` | [`arrays::JsonRecord`] / [`arrays::JsonRecordCbor`] |
//! | `coap-content-format-or-media-type` | [`choices::CoapContentFormatOrMediaType`] |
//! | `cmw-indicator = uint .bits cm-type` | [`choices::CmwIndicator`] |
//! | `cm-type` | [`choices::CmType`] |
//!
//! ### Collections and Tags ([Section 3.2](https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-3.2))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `cbor-collection = {? "__cmwc_t", + label => cbor-cmw}` | [`maps::CborCollection`] |
//! | `json-collection = {? "__cmwc_t", + label => json-cmw}` | [`maps::JsonCollection`] |
//! | `$cbor-tag /= #6.1668547091(cbor-collection)` | [`choices::CborCmw::TagCollection`] |
//! | `$cbor-tag /= #6.1668547092(COSE_Sign1)` | [`choices::CborCmw::TagSigned`] |
//! | `$cbor-tag /= #6.1668547093(bstr)` | [`choices::CborCmw::TagCmwJsonCollectionData`] |
//! | `$cbor-tag /= #6.1668547094(bstr)` | [`choices::CborCmw::TagCmwJwsData`] |
//! | `tag-cm-data<tn> = #6.<tn>(bytes)` | [`choices::CborCmw::TagData`] |
//!
//! ### Signed CMW ([Section 4.1](https://datatracker.ietf.org/doc/html/draft-ietf-rats-msg-wrap-23#section-4.1))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `signed-cbor-cmw` | [`signed::SignedCborCmw`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;
pub mod signed;
