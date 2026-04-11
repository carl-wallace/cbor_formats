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
//! [RFC 9393 Section 2.10](https://datatracker.ietf.org/doc/html/rfc9393#section-2.10)
//! to their Rust implementations, organized by specification section.
//!
//! ### CoSWID Tag ([Section 2.3](https://datatracker.ietf.org/doc/html/rfc9393#section-2.3))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `concise-swid-tag` | [`maps::ConciseSwidTag`] / [`maps::ConciseSwidTagCbor`] |
//!
//! ### Entity ([Section 2.6](https://datatracker.ietf.org/doc/html/rfc9393#section-2.6))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `entity-entry` | [`maps::EntityEntry`] / [`maps::EntityEntryCbor`] |
//! | `one-or-more<entity-entry>` | [`maps::OneOrMoreEntityEntry`] / [`maps::OneOrMoreEntityEntryCbor`] |
//! | `$role` / known roles | [`choices::Role`] / [`choices::RoleKnown`] |
//! | `one-or-more<role>` | [`choices::OneOrMoreRole`] |
//!
//! ### Links ([Section 2.7](https://datatracker.ietf.org/doc/html/rfc9393#section-2.7))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `link-entry` | [`maps::LinkEntry`] / [`maps::LinkEntryCbor`] |
//! | `one-or-more<link-entry>` | [`maps::OneOrMoreLinkEntry`] / [`maps::OneOrMoreLinkEntryCbor`] |
//! | `$ownership` / known values | [`choices::Ownership`] / [`choices::OwnershipKnown`] |
//! | `$rel` / known values | [`choices::Rel`] / [`choices::RelKnown`] |
//! | `$use-choice` / known values | [`choices::UseChoice`] / [`choices::UseChoiceKnown`] |
//!
//! ### Software Meta ([Section 2.8](https://datatracker.ietf.org/doc/html/rfc9393#section-2.8))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `software-meta-entry` | [`maps::SoftwareMetaEntry`] / [`maps::SoftwareMetaEntryCbor`] |
//! | `one-or-more<software-meta-entry>` | [`maps::OneOrMoreSoftwareMetaEntry`] / [`maps::OneOrMoreSoftwareMetaEntryCbor`] |
//!
//! ### Resources and Evidence ([Section 2.9](https://datatracker.ietf.org/doc/html/rfc9393#section-2.9))
//!
//! | CDDL | Rust |
//! |------|------|
//! | `path-elements-group` | [`maps::PathElementsGroup`] / [`maps::PathElementsGroupCbor`] |
//! | `resource-collection` | [`maps::ResourceCollection`] / [`maps::ResourceCollectionCbor`] |
//! | `file-entry` | [`maps::FileEntry`] / [`maps::FileEntryCbor`] |
//! | `one-or-more<file-entry>` | [`maps::OneOrMoreFileEntry`] / [`maps::OneOrMoreFileEntryCbor`] |
//! | `directory-entry` | [`maps::DirectoryEntry`] / [`maps::DirectoryEntryCbor`] |
//! | `one-or-more<directory-entry>` | [`maps::OneOrMoreDirectoryEntry`] / [`maps::OneOrMoreDirectoryEntryCbor`] |
//! | `process-entry` | [`maps::ProcessEntry`] / [`maps::ProcessEntryCbor`] |
//! | `one-or-more<process-entry>` | [`maps::OneOrMoreProcessEntry`] / [`maps::OneOrMoreProcessEntryCbor`] |
//! | `resource-entry` | [`maps::ResourceEntry`] / [`maps::ResourceEntryCbor`] |
//! | `one-or-more<resource-entry>` | [`maps::OneOrMoreResourceEntry`] / [`maps::OneOrMoreResourceEntryCbor`] |
//! | `payload-entry` | [`maps::PayloadEntry`] / [`maps::PayloadEntryCbor`] |
//! | `evidence-entry` | [`maps::EvidenceEntry`] / [`maps::EvidenceEntryCbor`] |
//! | `payload-or-evidence` | [`choices::PayloadOrEvidence`] |
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;

// any-uri = uri

// any-attribute = (
//   label => one-or-more<text> / one-or-more<int>
// )
//
// one-or-more<T> = T / [ 2* T ]
//
// global-attributes = (
//   ? lang => text,
//   * any-attribute,
// )
//
// path-elements-group = ( ? directory => one-or-more<directory-entry>,
//                         ? file => one-or-more<file-entry>,
//                       )
//
// resource-collection = (
//   path-elements-group,
//   ? process => one-or-more<process-entry>,
//   ? resource => one-or-more<resource-entry>,
//   * $$resource-collection-extension,
// )
//
// filesystem-item = (
//   ? key => bool,
//   ? location => text,
//   fs-name => text,
//   ? root => text,
// )
//
// integer-time = #6.1(int)
//

// ; "global map member" integer indexes
// activation-status = 43
// artifact = 37
// channel-type = 44
// colloquial-version = 45
// date = 35
// description = 46
// device-id = 36
// directory = 16
// edition = 47
// entitlement-data-required = 48
// entitlement-key = 49
// entity-name = 31
// file = 17
// file-version = 21
// fs-name = 24
// generator = 50
// hash = 7
// href = 38
// key = 22
// lang = 15
// location = 23
// media-type = 41
// ownership = 39
// path-elements = 26
// persistent-id = 51
// pid = 28
// process = 18
// process-name = 27
// product = 52
// product-family = 53
// reg-id = 32
// rel = 40
// resource = 19
// revision = 54
// role = 33
// root = 25
// size = 20
// summary = 55
// thumbprint = 34
// type = 29
// unspsc-code = 56
// unspsc-version = 57
// use = 42
//
