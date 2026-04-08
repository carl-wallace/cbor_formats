#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

extern crate alloc;

pub mod algorithm;
pub mod crypto;
pub mod encrypt;
pub mod error;
pub mod helpers;
pub mod jwk;
pub mod keys;
pub mod mac;
pub mod sign;
