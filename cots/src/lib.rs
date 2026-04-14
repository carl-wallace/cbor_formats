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
//! [draft-ietf-rats-concise-ta-stores-02 Section 4](https://datatracker.ietf.org/doc/html/draft-ietf-rats-concise-ta-stores-02#section-4)
//! to their Rust implementations.
//!
//! ### Concise TA Stores ([Section 4](https://datatracker.ietf.org/doc/html/draft-ietf-rats-concise-ta-stores-02#section-4))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `concise-ta-stores = [+ concise-ta-store-map]` | [`arrays::ConciseTaStores`] / [`arrays::ConciseTaStoresCbor`] |
//! | `concise-ta-store-map` | [`maps::ConciseTaStoreMap`] / [`maps::ConciseTaStoreMapCbor`] |
//! | `tas-list-purpose` | [`choices::TasListPurpose`] |
//! | `environment-group-list` | [`arrays::EnvironmentGroupList`] / [`arrays::EnvironmentGroupListCbor`] |
//! | `environment-group-list-map` | [`maps::EnvironmentGroupListMap`] / [`maps::EnvironmentGroupListMapCbor`] |
//! | `abbreviated-swid-tag` | [`maps::AbbreviatedSwidTag`] / [`maps::AbbreviatedSwidTagCbor`] |
//! | `cas-and-tas-map` | [`maps::CasAndTasMap`] / [`maps::CasAndTasMapCbor`] |
//! | `trust-anchor` | [`arrays::TrustAnchor`] / [`arrays::TrustAnchorCbor`] |
//! | `$pkix-ta-type` | [`choices::PkixTaType`] / [`choices::PkixTaTypeKnown`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// pkix-cert-data = bstr

// named-ta-store = tstr

pub mod arrays;
pub mod choices;
pub mod maps;
