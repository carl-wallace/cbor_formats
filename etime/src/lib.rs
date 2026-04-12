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
//! [RFC 9581](https://www.rfc-editor.org/rfc/rfc9581.html)
//! to their Rust implementations.
//!
//! ### Extended Time ([Section 3](https://www.rfc-editor.org/rfc/rfc9581.html#section-3))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `etime = #6.1001({* (int/tstr) => any})` | [`maps::EtimeMapCbor`] (wrapped in tag 1001) |
//! | `duration = #6.1002({* (int/tstr) => any})` | [`maps::DurationMapCbor`] (wrapped in tag 1002) |
//! | `period = #6.1003([~etime/null, ~etime/null, ?~duration])` | [`maps::PeriodCbor`] (wrapped in tag 1003) |
//!
//! ### Time Map Keys ([Section 3](https://www.rfc-editor.org/rfc/rfc9581.html#section-3))
//!
//! | CDDL | Rust |
//! |------|------|
//! | base time (keys 1, 4, 5) | [`choices::BaseTime`] |
//! | `$ETIME-TIMESCALE` | [`choices::EtimeTimescale`] |
//! | `ClockQuality-group` | fields on [`maps::EtimeMap`] |
//! | `time-zone-info` | [`choices::TimeZoneInfo`] |
//! | `suffix-info-map` | [`choices::SuffixInfoMap`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod choices;
pub mod maps;
