#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//!
//! ## CDDL-to-Rust Type Mapping
//!
//! The following table maps CDDL productions from
//! [RFC 9711 Section 7.3](https://datatracker.ietf.org/doc/html/rfc9711#section-7.3)
//! to their Rust implementations.
//!
//! ### Array types ([`arrays`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `Detached-Submodule-Digest` | [`arrays::DetachedSubmoduleDigest`] / [`arrays::DetachedSubmoduleDigestCbor`] |
//! | `dloa-type` | [`arrays::DloaType`] / [`arrays::DloaTypeCbor`] |
//! | `hardware-version-type` | [`arrays::HardwareVersionType`] / [`arrays::HardwareVersionTypeCbor`] |
//! | `sw-version-type` | [`arrays::SwVersionType`] / [`arrays::SwVersionTypeCbor`] |
//! | `individual-result` | [`arrays::IndividualResult`] / [`arrays::IndividualResultCbor`] |
//! | `manifests-type` | [`arrays::ManifestsType`] / [`arrays::ManifestsTypeCbor`] |
//! | `manifest-format` | [`arrays::ManifestFormat`] / [`arrays::ManifestFormatCbor`] |
//! | `measurements-type` | [`arrays::MeasurementsType`] / [`arrays::MeasurementsTypeCbor`] |
//! | `measurements-format` | [`arrays::MeasurementsFormat`] / [`arrays::MeasurementsFormatCbor`] |
//! | `measurement-results-group` | [`arrays::MeasurementResultsGroup`] / [`arrays::MeasurementResultsGroupCbor`] |
//! | `[ + measurement-results-group ]` | [`arrays::MeasurementResultsGroupArray`] / [`arrays::MeasurementResultsGroupArrayCbor`] |
//! | `Nested-Token` (JSON) | [`arrays::NestedToken`] |
//! | `Nested-Token` (CBOR) | [`arrays::NestedTokenCbor`] |
//! | `Wrapped-Claims-Set` (JSON) | [`arrays::WrappedClaimsSet`] |
//! | `Wrapped-Claims-Set` (CBOR) | [`arrays::WrappedClaimsSetCbor`] |
//! | `Detached-EAT-Bundle` | [`arrays::DetachedEatBundle`] / [`arrays::DetachedEatBundleCbor`] |
//!
//! ### Choice types ([`choices`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `debug-status-type` | [`choices::DebugStatusType`] |
//! | `intended-use-type` | [`choices::IntendedUseType`] |
//! | `oemid` | [`choices::Oemid`] |
//! | `result-type` | [`choices::ResultType`] |
//!
//! ### Map types ([`maps`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `Claims-Set` | [`maps::ClaimsSetClaims`] / [`maps::ClaimsSetClaimsCbor`] |
//! | `location-type` | [`maps::LocationType`] / [`maps::LocationTypeCbor`] |
//! | `sueids-type` | [`maps::SueidsType`] / [`maps::SueidsTypeCbor`] |
//!
//! ### CBOR-specific types ([`cbor_specific`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `CBOR-Selector` | [`cbor_specific::SelectorCbor`] |
//! | `Submodule` (CBOR) | [`cbor_specific::SubmoduleCbor`] |
//!
//! ### JSON-specific types ([`json_specific`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `$JSON-Selector-Type` | [`json_specific::JsonSelectorType`] |
//! | `$JSON-Selector-Value` | [`json_specific::JsonSelectorValue`] |
//! | `JSON-Selector` | [`json_specific::JsonSelector`] |
//! | `$JSON-Selector-Value` (for-deb variant) | [`json_specific::JsonSelectorForDebValue`] |
//! | `Selector-For-Deb` | [`json_specific::SelectorForDeb`] |
//! | `Submodule` (JSON) | [`json_specific::Submodule`] |
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod cbor_specific;
pub mod choices;
pub mod json_specific;
pub mod maps;
