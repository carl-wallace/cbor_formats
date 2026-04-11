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
//! [draft-ietf-rats-coserv-05 Appendix A](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#appendix-A)
//! to their Rust implementations, organized by specification section.
//!
//! ### CoSERV Message ([Section 4](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `coserv` | [`maps::CoservMap`] / [`maps::CoservMapCbor`] |
//! | `comid.oid-type / ~uri` | [`choices::CoservProfile`] / [`choices::CoservProfileCbor`] |
//! | `tdate` | [`maps::Tdate`] / [`maps::TdateCbor`] |
//!
//! ### Queries ([Section 4.3](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.3))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `query` | [`maps::QueryMap`] / [`maps::QueryMapCbor`] |
//! | `$artifact-type` | [`choices::ArtifactType`] |
//! | `environment-selector-map` | [`maps::EnvironmentSelectorMap`] / [`maps::EnvironmentSelectorMapCbor`] |
//! | `stateful-class` | [`arrays::StatefulClass`] / [`arrays::StatefulClassCbor`] |
//! | `stateful-instance` | [`arrays::StatefulInstance`] / [`arrays::StatefulInstanceCbor`] |
//! | `stateful-group` | [`arrays::StatefulGroup`] / [`arrays::StatefulGroupCbor`] |
//!
//! ### Results ([Section 4.4](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.4))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `results` | [`maps::ResultsMap`] / [`maps::ResultsMapCbor`] |
//! | `$result-type` | [`choices::ResultType`] |
//! | `refval-quad` | [`maps::RefvalQuadMap`] / [`maps::RefvalQuadMapCbor`] |
//! | `endval-quad` | [`maps::EndvalQuadMap`] / [`maps::EndvalQuadMapCbor`] |
//! | `cond-endval-quad` | [`maps::CondEndvalQuadMap`] / [`maps::CondEndvalQuadMapCbor`] |
//! | `ak-quad` | [`maps::AkQuadMap`] / [`maps::AkQuadMapCbor`] |
//! | `cots-stmt` | [`maps::CotsStmtMap`] / [`maps::CotsStmtMapCbor`] |
//!
//! ### Signed CoSERV ([Section 4.6](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.6))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `signed-coserv` | [`signed::SignedCoserv`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;
pub mod signed;
