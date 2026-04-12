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
//! [draft-ietf-rats-epoch-markers-03](https://datatracker.ietf.org/doc/html/draft-ietf-rats-epoch-markers-03)
//! to their Rust implementations.
//!
//! ### Choices ([`choices`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `epoch-marker` / `$tagged-epoch-id` | [`choices::EpochMarker`] |
//! | `cbor-time` | [`choices::CborTime`] |
//! | `epoch-tick` | [`choices::EpochTick`] |
//!
//! ### Maps ([`maps`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `TST-info-based-on-CBOR-time-tag` | [`maps::TstInfoCborTimeTag`] / [`maps::TstInfoCborTimeTagCbor`] |
//! | `profiled-etime` | [`maps::ProfiledEtime`] / [`maps::ProfiledEtimeCbor`] |
//!
//! ### Arrays ([`arrays`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `MessageImprint` | [`arrays::MessageImprint`] / [`arrays::MessageImprintCbor`] |
//! | `GeneralName` | [`arrays::GeneralName`] / [`arrays::GeneralNameCbor`] |
//! | `epoch-tick-list` | [`arrays::EpochTickList`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;
