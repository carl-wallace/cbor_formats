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
//! [RFC 9711 Section 7.3](https://datatracker.ietf.org/doc/html/rfc9711#section-7.3)
//! to their Rust implementations, organized by specification section.
//!
//! ### Claims-Set ([Section 4.2](https://datatracker.ietf.org/doc/html/rfc9711#section-4.2))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `Claims-Set` | [`maps::ClaimsSetClaims`] / [`maps::ClaimsSetClaimsCbor`] |
//! | `sueids-type` | [`maps::SueidsType`] / [`maps::SueidsTypeCbor`] |
//! | `oemid` | [`choices::Oemid`] |
//! | `hardware-version-type` | [`arrays::HardwareVersionType`] / [`arrays::HardwareVersionTypeCbor`] |
//! | `sw-version-type` | [`arrays::SwVersionType`] / [`arrays::SwVersionTypeCbor`] |
//! | `debug-status-type` | [`choices::DebugStatusType`] |
//! | `location-type` | [`maps::LocationType`] / [`maps::LocationTypeCbor`] |
//! | `dloa-type` | [`arrays::DloaType`] / [`arrays::DloaTypeCbor`] |
//!
//! ### Manifests and Measurements ([Section 4.2.15–4.2.17](https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.15))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `manifests-type` | [`arrays::ManifestsType`] / [`arrays::ManifestsTypeCbor`] |
//! | `manifest-format` | [`arrays::ManifestFormat`] / [`arrays::ManifestFormatCbor`] |
//! | `measurements-type` | [`arrays::MeasurementsType`] / [`arrays::MeasurementsTypeCbor`] |
//! | `measurements-format` | [`arrays::MeasurementsFormat`] / [`arrays::MeasurementsFormatCbor`] |
//! | `measurement-results-group` | [`arrays::MeasurementResultsGroup`] / [`arrays::MeasurementResultsGroupCbor`] |
//! | `[ + measurement-results-group ]` | [`arrays::MeasurementResultsGroupArray`] / [`arrays::MeasurementResultsGroupArrayCbor`] |
//! | `individual-result` | [`arrays::IndividualResult`] / [`arrays::IndividualResultCbor`] |
//! | `result-type` | [`choices::ResultType`] |
//! | `intended-use-type` | [`choices::IntendedUseType`] |
//!
//! ### Submodules and Nested Tokens ([Section 4.2.18](https://datatracker.ietf.org/doc/html/rfc9711#section-4.2.18))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `Nested-Token` (JSON) | [`arrays::NestedToken`] |
//! | `Nested-Token` (CBOR) | [`arrays::NestedTokenCbor`] |
//! | `Detached-Submodule-Digest` | [`arrays::DetachedSubmoduleDigest`] / [`arrays::DetachedSubmoduleDigestCbor`] |
//! | `CBOR-Selector` | [`cbor_specific::SelectorCbor`] |
//! | `Submodule` (CBOR) | [`cbor_specific::SubmoduleCbor`] |
//! | `{ + text => Submodule }` | [`json_specific::SubmodsMap`] / [`cbor_specific::SubmodsMapCbor`] |
//! | `$JSON-Selector-Type` | [`json_specific::JsonSelectorType`] |
//! | `$JSON-Selector-Value` | [`json_specific::JsonSelectorValue`] |
//! | `JSON-Selector` | [`json_specific::JsonSelector`] |
//! | `$JSON-Selector-Value` (for-deb variant) | [`json_specific::JsonSelectorForDebValue`] |
//! | `Selector-For-Deb` | [`json_specific::SelectorForDeb`] |
//! | `Submodule` (JSON) | [`json_specific::Submodule`] |
//!
//! ### Wrapped Claims Sets and Bundles ([Section 5](https://datatracker.ietf.org/doc/html/rfc9711#section-5))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `Wrapped-Claims-Set` (JSON) | [`arrays::WrappedClaimsSet`] |
//! | `Wrapped-Claims-Set` (CBOR) | [`arrays::WrappedClaimsSetCbor`] |
//! | `Detached-EAT-Bundle` | [`arrays::DetachedEatBundle`] / [`arrays::DetachedEatBundleCbor`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod cbor_specific;
pub mod choices;
pub mod json_specific;
pub mod maps;
