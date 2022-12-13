#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
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
