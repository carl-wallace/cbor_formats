#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// pkix-cert-data = bstr

// named-ta-store = tstr

pub mod arrays;
pub mod choices;
pub mod maps;
