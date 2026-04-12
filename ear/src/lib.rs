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
//! [draft-ietf-rats-ear-03](https://datatracker.ietf.org/doc/html/draft-ietf-rats-ear-03)
//! to their Rust implementations.
//!
//! ### Maps ([`maps`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `EAR` | [`maps::Ear`] / [`maps::EarCbor`] |
//! | `EAR-appraisal` | [`maps::EarAppraisal`] / [`maps::EarAppraisalCbor`] |
//! | `{ + text => EAR-appraisal }` | [`maps::EarSubmods`] / [`maps::EarSubmodsCbor`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod maps;
