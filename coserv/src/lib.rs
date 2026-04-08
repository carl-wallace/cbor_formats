#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//!
//! ## CDDL-to-Rust Type Mapping
//!
//! The following table maps CDDL productions from
//! [draft-ietf-rats-coserv-05 Appendix A](https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#appendix-A)
//! to their Rust implementations.
//!
//! ### Array types ([`arrays`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `stateful-class` | [`arrays::StatefulClass`] / [`arrays::StatefulClassCbor`] |
//! | `stateful-instance` | [`arrays::StatefulInstance`] / [`arrays::StatefulInstanceCbor`] |
//! | `stateful-group` | [`arrays::StatefulGroup`] / [`arrays::StatefulGroupCbor`] |
//!
//! ### Choice types ([`choices`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `$artifact-type` | [`choices::ArtifactType`] |
//! | `$result-type` | [`choices::ResultType`] |
//!
//! ### Map types ([`maps`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `coserv-map` | [`maps::CoservMap`] / [`maps::CoservMapCbor`] |
//! | `query-map` | [`maps::QueryMap`] / [`maps::QueryMapCbor`] |
//! | `environment-selector-map` | [`maps::EnvironmentSelectorMap`] / [`maps::EnvironmentSelectorMapCbor`] |
//! | `results-map` | [`maps::ResultsMap`] / [`maps::ResultsMapCbor`] |
//! | `refval-quad-map` | [`maps::RefvalQuadMap`] / [`maps::RefvalQuadMapCbor`] |
//! | `endval-quad-map` | [`maps::EndvalQuadMap`] / [`maps::EndvalQuadMapCbor`] |
//! | `cond-endval-quad-map` | [`maps::CondEndvalQuadMap`] / [`maps::CondEndvalQuadMapCbor`] |
//! | `ak-quad-map` | [`maps::AkQuadMap`] / [`maps::AkQuadMapCbor`] |
//! | `cots-stmt-map` | [`maps::CotsStmtMap`] / [`maps::CotsStmtMapCbor`] |
//! | `tdate` | [`maps::Tdate`] / [`maps::TdateCbor`] |
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;
