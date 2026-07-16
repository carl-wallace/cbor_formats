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
//! [draft-ietf-rats-ar4si-09](https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09)
//! to their Rust implementations.
//!
//! ### Maps ([`maps`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `verifier-id` | [`maps::VerifierId`] / [`maps::VerifierIdCbor`] |
//! | `trustworthiness-vector` | [`maps::TrustworthinessVector`] / [`maps::TrustworthinessVectorCbor`] |
//!
//! ### Choices ([`choices`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `trustworthiness-tier` | [`choices::TrustworthinessTier`] |
//! | `trustworthiness-claim` | `i8` |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod choices;
pub mod maps;
