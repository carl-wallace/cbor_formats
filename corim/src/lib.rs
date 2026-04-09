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
//! to their Rust implementations.
//!
//! ### Array types ([`arrays`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `reference-triple-record` | [`arrays::ReferenceTripleRecord`] / [`arrays::ReferenceTripleRecordCbor`] |
//! | `endorsed-triple-record` | [`arrays::EndorsedTripleRecord`] / [`arrays::EndorsedTripleRecordCbor`] |
//! | `identity-triple-record` | [`arrays::IdentityTripleRecord`] / [`arrays::IdentityTripleRecordCbor`] |
//! | `attest-key-triple-record` | [`arrays::AttestKeyTripleRecord`] / [`arrays::AttestKeyTripleRecordCbor`] |
//! | `coswid-triple-record` | [`arrays::CoswidTripleRecord`] / [`arrays::CoswidTripleRecordCbor`] |
//! | `domain-dependency-triple-record` | [`arrays::DomainDependencyTripleRecord`] / [`arrays::DomainDependencyTripleRecordCbor`] |
//! | `domain-membership-triple-record` | [`arrays::DomainMembershipTripleRecord`] / [`arrays::DomainMembershipTripleRecordCbor`] |
//! | `conditional-endorsement-triple-record` | [`arrays::ConditionalEndorsementTripleRecord`] / [`arrays::ConditionalEndorsementTripleRecordCbor`] |
//! | `stateful-environment-record` | [`arrays::StatefulEnvironmentRecord`] / [`arrays::StatefulEnvironmentRecordCbor`] |
//! | `conditional-series-record` | [`arrays::ConditionalSeriesRecord`] / [`arrays::ConditionalSeriesRecordCbor`] |
//! | `conditional-endorsement-series-triple-record` | [`arrays::ConditionalEndorsementSeriesTripleRecord`] / [`arrays::ConditionalEndorsementSeriesTripleRecordCbor`] |
//! | `conditional-endorsement-series-condition` | [`arrays::ConditionalEndorsementSeriesCondition`] / [`arrays::ConditionalEndorsementSeriesConditionCbor`] |
//!
//! ### Choice types ([`choices`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `tagged-coswid-type = #6.505(concise-swid-tag)` | [`choices::TaggedCoswid`] / [`choices::TaggedCoswidCbor`] |
//! | `tagged-concise-mid-tag = #6.506(concise-mid-tag)` | [`choices::TaggedComid`] / [`choices::TaggedComidCbor`] |
//! | `$concise-tag-type-choice` | [`choices::ConciseTagTypeChoice`] |
//! | `$class-id-type-choice` | [`choices::ClassIdTypeChoice`] / [`choices::ClassIdTypeChoiceCbor`] |
//! | `$corim-id-type-choice` | [`choices::CorimIdTypeChoice`] |
//! | `$corim-role-type-choice` | [`choices::CorimRoleTypeChoice`] / [`choices::CorimRoleTypeChoiceCbor`] |
//! | `$crypto-key-type-choice` | [`choices::CryptoKeyTypeChoice`] / [`choices::CryptoKeyTypeChoiceCbor`] |
//! | `$domain-type-choice` | [`choices::DomainTypeChoice`] |
//! | `$entity-name-type-choice` | [`choices::EntityNameTypeChoice`] |
//! | `$group-id-type-choice` | [`choices::GroupIdTypeChoice`] |
//! | `$instance-id-type-choice` | [`choices::InstanceIdTypeChoice`] |
//! | `$measured-element-type-choice` | [`choices::MeasuredElementTypeChoice`] / [`choices::MeasuredElementTypeChoiceCbor`] |
//! | `$profile-type-choice` | [`choices::ProfileTypeChoice`] / [`choices::ProfileTypeChoiceCbor`] |
//! | `$svn-type-choice` | [`choices::SvnTypeChoice`] |
//! | `$tag-id-type-choice` | [`choices::TagIdTypeChoice`] / [`choices::TagIdTypeChoiceCbor`] |
//! | `$tag-rel-type-choice` | [`choices::TagRelTypeChoice`] |
//! | `$tag-version-type` | [`choices::TagVersionType`] |
//! | `$raw-value-type-choice` | [`choices::RawValueTypeChoice`] / [`choices::RawValueTypeChoiceCbor`] |
//! | `$int-range-type-choice` | [`choices::IntRangeTypeChoice`] / [`choices::IntRangeTypeChoiceCbor`] |
//!
//! ### Map types ([`maps`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `unsigned-corim-map` | [`maps::CorimMap`] / [`maps::CorimMapCbor`] |
//! | `corim-meta-map` | [`maps::CorimMetaMap`] / [`maps::CorimMetaMapCbor`] |
//! | `corim-signer-map` | [`maps::CorimSignerMap`] / [`maps::CorimSignerMapCbor`] |
//! | `corim-locator-map` | [`maps::CorimLocatorMap`] / [`maps::CorimLocatorMapCbor`] |
//! | `concise-mid-tag` | [`maps::ConciseMidTag`] / [`maps::ConciseMidTagCbor`] |
//! | `class-map` | [`maps::ClassMap`] / [`maps::ClassMapCbor`] |
//! | `environment-map` | [`maps::EnvironmentMap`] / [`maps::EnvironmentMapCbor`] |
//! | `measurement-map` | [`maps::MeasurementMap`] / [`maps::MeasurementMapCbor`] |
//! | `measurement-values-map` | [`maps::MeasurementValuesMap`] / [`maps::MeasurementValuesMapCbor`] |
//! | `flags-map` | [`maps::FlagsMap`] / [`maps::FlagsMapCbor`] |
//! | `entity-map` | [`maps::EntityMap`] / [`maps::EntityMapCbor`] |
//! | `corim-entity-map` | [`maps::CorimEntityMap`] / [`maps::CorimEntityMapCbor`] |
//! | `comid-entity-map` | [`maps::ComidEntityMap`] / [`maps::ComidEntityMapCbor`] |
//! | `tag-identity-map` | [`maps::TagIdentityMap`] / [`maps::TagIdentityMapCbor`] |
//! | `linked-tag-map` | [`maps::LinkedTagMap`] / [`maps::LinkedTagMapCbor`] |
//! | `triples-map` | [`maps::TriplesMap`] / [`maps::TriplesMapCbor`] |
//! | `validity-map` | [`maps::ValidityMap`] / [`maps::ValidityMapCbor`] |
//! | `version-map` | [`maps::VersionMap`] / [`maps::VersionMapCbor`] |
//! | `protected-corim-header-map` | [`maps::ProtectedCorimHeaderMap`] / [`maps::ProtectedCorimHeaderMapCbor`] |
//! | `attest-key-conditions-map` | [`maps::AttestKeyConditionsMap`] / [`maps::AttestKeyConditionsMapCbor`] |
//! | `concise-tl-tag` | [`maps::ConciseTlTag`] / [`maps::ConciseTlTagCbor`] |
//!
//! ### Type aliases ([root module])
//!
//! | CDDL | Rust |
//! |------|------|
//! | `tagged-unsigned-corim-map` (`#6.501`) | [`TaggedUnsignedCorimMap`] |
//! | `raw-value-mask-type` | [`RawValueMaskType`] |
//!
//! Additional CoRIM-defined types are in the [`common`] crate:
//! `digests-type`, `integrity-registers`, `tagged-masked-raw-value`,
//! and the various `tagged-*` key/cert types. See the
//! [`common` crate docs](common) for the full list.
//!
//! ### Signed types ([`signed`] module)
//!
//! | CDDL | Rust |
//! |------|------|
//! | `COSE-Sign1-corim` | [`signed::SignedCorim`] |
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
