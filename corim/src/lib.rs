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
//! [draft-ietf-rats-corim-10 Appendix A](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#appendix-A)
//! to their Rust implementations, organized by specification section.
//!
//! ### CoRIM Map ([Section 4.1](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `tagged-unsigned-corim-map` (`#6.501`) | [`TaggedUnsignedCorimMap`] |
//! | `unsigned-corim-map` | [`maps::CorimMap`] / [`maps::CorimMapCbor`] |
//! | `$corim-id-type-choice` | [`choices::CorimIdTypeChoice`] |
//! | `$concise-tag-type-choice` | [`choices::ConciseTagTypeChoice`] |
//! | `tagged-coswid-type = #6.505(concise-swid-tag)` | [`choices::TaggedCoswid`] / [`choices::TaggedCoswidCbor`] |
//! | `tagged-concise-mid-tag = #6.506(concise-mid-tag)` | [`choices::TaggedComid`] / [`choices::TaggedComidCbor`] |
//! | `corim-locator-map` | [`maps::CorimLocatorMap`] / [`maps::CorimLocatorMapCbor`] |
//! | `$profile-type-choice` | [`choices::ProfileTypeChoice`] / [`choices::ProfileTypeChoiceCbor`] |
//! | `$corim-role-type-choice` | [`choices::CorimRoleTypeChoice`] / [`choices::CorimRoleTypeChoiceCbor`] |
//!
//! ### Signed CoRIM ([Section 4.2](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.2))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `COSE-Sign1-corim` | [`signed::SignedCorim`] |
//! | `protected-corim-header-map` | [`maps::ProtectedCorimHeaderMap`] / [`maps::ProtectedCorimHeaderMapCbor`] |
//! | `corim-meta-map` | [`maps::CorimMetaMap`] / [`maps::CorimMetaMapCbor`] |
//! | `corim-signer-map` | [`maps::CorimSignerMap`] / [`maps::CorimSignerMapCbor`] |
//!
//! ### Concise Module Identifier (CoMID) Tag ([Section 5.1](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `concise-mid-tag` | [`maps::ConciseMidTag`] / [`maps::ConciseMidTagCbor`] |
//! | `tag-identity-map` | [`maps::TagIdentityMap`] / [`maps::TagIdentityMapCbor`] |
//! | `$tag-id-type-choice` | [`choices::TagIdTypeChoice`] / [`choices::TagIdTypeChoiceCbor`] |
//! | `$tag-version-type` | [`choices::TagVersionType`] |
//! | `linked-tag-map` | [`maps::LinkedTagMap`] / [`maps::LinkedTagMapCbor`] |
//! | `$tag-rel-type-choice` | [`choices::TagRelTypeChoice`] |
//!
//! ### Environments and Measurements ([Section 5.1.4](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.4))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `triples-map` | [`maps::TriplesMap`] / [`maps::TriplesMapCbor`] |
//! | `environment-map` | [`maps::EnvironmentMap`] / [`maps::EnvironmentMapCbor`] |
//! | `class-map` | [`maps::ClassMap`] / [`maps::ClassMapCbor`] |
//! | `$class-id-type-choice` | [`choices::ClassIdTypeChoice`] / [`choices::ClassIdTypeChoiceCbor`] |
//! | `$instance-id-type-choice` | [`choices::InstanceIdTypeChoice`] |
//! | `$group-id-type-choice` | [`choices::GroupIdTypeChoice`] |
//! | `measurement-map` | [`maps::MeasurementMap`] / [`maps::MeasurementMapCbor`] |
//! | `$measured-element-type-choice` | [`choices::MeasuredElementTypeChoice`] / [`choices::MeasuredElementTypeChoiceCbor`] |
//! | `measurement-values-map` | [`maps::MeasurementValuesMap`] / [`maps::MeasurementValuesMapCbor`] |
//! | `version-map` | [`maps::VersionMap`] / [`maps::VersionMapCbor`] |
//! | `$svn-type-choice` | [`choices::SvnTypeChoice`] |
//! | `flags-map` | [`maps::FlagsMap`] / [`maps::FlagsMapCbor`] |
//! | `$raw-value-type-choice` | [`choices::RawValueTypeChoice`] / [`choices::RawValueTypeChoiceCbor`] |
//! | `raw-value-mask-type` | [`RawValueMaskType`] |
//! | `$crypto-key-type-choice` | [`choices::CryptoKeyTypeChoice`] / [`choices::CryptoKeyTypeChoiceCbor`] |
//! | `$int-range-type-choice` | [`choices::IntRangeTypeChoice`] / [`choices::IntRangeTypeChoiceCbor`] |
//!
//! ### Triple Records ([Section 5.1.5–5.1.12](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-5.1.5))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `reference-triple-record` | [`arrays::ReferenceTripleRecord`] / [`arrays::ReferenceTripleRecordCbor`] |
//! | `endorsed-triple-record` | [`arrays::EndorsedTripleRecord`] / [`arrays::EndorsedTripleRecordCbor`] |
//! | `conditional-endorsement-triple-record` | [`arrays::ConditionalEndorsementTripleRecord`] / [`arrays::ConditionalEndorsementTripleRecordCbor`] |
//! | `stateful-environment-record` | [`arrays::StatefulEnvironmentRecord`] / [`arrays::StatefulEnvironmentRecordCbor`] |
//! | `conditional-series-record` | [`arrays::ConditionalSeriesRecord`] / [`arrays::ConditionalSeriesRecordCbor`] |
//! | `conditional-endorsement-series-triple-record` | [`arrays::ConditionalEndorsementSeriesTripleRecord`] / [`arrays::ConditionalEndorsementSeriesTripleRecordCbor`] |
//! | `conditional-endorsement-series-condition` | [`arrays::ConditionalEndorsementSeriesCondition`] / [`arrays::ConditionalEndorsementSeriesConditionCbor`] |
//! | `identity-triple-record` | [`arrays::IdentityTripleRecord`] / [`arrays::IdentityTripleRecordCbor`] |
//! | `attest-key-triple-record` | [`arrays::AttestKeyTripleRecord`] / [`arrays::AttestKeyTripleRecordCbor`] |
//! | `attest-key-conditions-map` | [`maps::AttestKeyConditionsMap`] / [`maps::AttestKeyConditionsMapCbor`] |
//! | `$domain-type-choice` | [`choices::DomainTypeChoice`] |
//! | `domain-membership-triple-record` | [`arrays::DomainMembershipTripleRecord`] / [`arrays::DomainMembershipTripleRecordCbor`] |
//! | `domain-dependency-triple-record` | [`arrays::DomainDependencyTripleRecord`] / [`arrays::DomainDependencyTripleRecordCbor`] |
//! | `coswid-triple-record` | [`arrays::CoswidTripleRecord`] / [`arrays::CoswidTripleRecordCbor`] |
//!
//! ### Trustworthiness Labels ([Section 6.1](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-6.1))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `concise-tl-tag` | [`maps::ConciseTlTag`] / [`maps::ConciseTlTagCbor`] |
//!
//! ### Entities and Validity ([Section 7.2–7.3](https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-7.2))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `entity-map` | [`maps::EntityMap`] / [`maps::EntityMapCbor`] |
//! | `corim-entity-map` | [`maps::CorimEntityMap`] / [`maps::CorimEntityMapCbor`] |
//! | `comid-entity-map` | [`maps::ComidEntityMap`] / [`maps::ComidEntityMapCbor`] |
//! | `$entity-name-type-choice` | [`choices::EntityNameTypeChoice`] |
//! | `validity-map` | [`maps::ValidityMap`] / [`maps::ValidityMapCbor`] |
//!
//! Additional CoRIM-defined types are in the [`common`] crate:
//! `digests-type`, `integrity-registers`, `tagged-masked-raw-value`,
//! and the various `tagged-*` key/cert types. See the
//! [`common` crate docs](common) for the full list.
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;
pub mod signed;

use alloc::vec::Vec;

use ciborium::tag::Required;
use maps::{CorimMapCbor, EntityMap, EntityMapCbor};

/// `tagged-unsigned-corim-map` = `#6.501(unsigned-corim-map)`.
///
/// See [CoRIM Section 4.1].
///
/// [CoRIM Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-corim-10#section-4.1
pub type TaggedUnsignedCorimMap = Required<CorimMapCbor, 501>;

/// The `corim-entity-map` instantiation of [`EntityMap`].
///
/// See [`maps::CorimEntityMap`].
pub type CorimEntityMap = EntityMap;

/// CBOR-encoded form of [`CorimEntityMap`].
pub type CorimEntityMapCbor = EntityMapCbor;

/// raw-value-mask-type = bytes (deprecated in draft-10, kept for backward compat)
pub type RawValueMaskType = Vec<u8>;

//    ip-addr-type-choice = ip4-addr-type / ip6-addr-type
//    ip4-addr-type = bytes .size 4
//    ip6-addr-type = bytes .size 16
//
//    mac-addr-type-choice = eui48-addr-type / eui64-addr-type
//    eui48-addr-type = bytes .size 6
//    eui64-addr-type = bytes .size 8
//
// svn = int
// min-svn = int
// tagged-svn = #6.552(svn)
// tagged-min-svn = #6.553(min-svn)
// svn-type-choice = tagged-svn / tagged-min-svn
//
// flags-type = bytes ;.bits operational-flags
//
// operational-flags = &(
//   not-configured: 0
//   not-secure: 1
//   recovery: 2
//   debug: 3
// )
//
// ip-addr-type-choice = ip4-addr-type / ip6-addr-type
// ip4-addr-type = bytes .size 4
// ip6-addr-type = bytes .size 16
//
// mac-addr-type-choice = eui48-addr-type / eui64-addr-type
// eui48-addr-type = bytes .size 6
// eui64-addr-type = bytes .size 8
//
// serial-number-type = text
//
// digests-type = [ + hash-entry ]
//
// ; non-empty<M> = (M) .within ({ + any => any })
// non-empty<M> = M .within ({ + any => any })
//
// cose-label = int / tstr
// cose-values = any
