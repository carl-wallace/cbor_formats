#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//!
//! ## CDDL-to-Rust Type Mapping
//!
//! The following table maps CDDL productions from
//! [RFC 9052 Section 1.4](https://datatracker.ietf.org/doc/html/rfc9052#section-1.4)
//! to their Rust implementations.
//!
//! | CDDL | Rust |
//! |------|------|
//! | `COSE_Sign` | [`arrays::CoseSign`] / [`arrays::CoseSignCbor`] |
//! | `COSE_Sign_Tagged = #6.98(COSE_Sign)` | [`arrays::TaggedCoseSign`] |
//! | `COSE_Signature` | [`arrays::CoseSignature`] / [`arrays::CoseSignatureCbor`] |
//! | `COSE_Sign1` | [`arrays::CoseSign1`] / [`arrays::CoseSign1Cbor`] |
//! | `COSE_Sign1_Tagged = #6.18(COSE_Sign1)` | [`arrays::TaggedCoseSign1`] |
//! | `Sig_structure` | [`arrays::SigStructure`] / [`arrays::SigStructureCbor`] |
//! | `COSE_Encrypt` | [`arrays::CoseEncrypt`] / [`arrays::CoseEncryptCbor`] |
//! | `COSE_Encrypt_Tagged = #6.96(COSE_Encrypt)` | [`arrays::TaggedCoseEncrypt`] |
//! | `COSE_recipient` | [`arrays::CoseRecipient`] / [`arrays::CoseRecipientCbor`] |
//! | `COSE_Encrypt0` | [`arrays::CoseEncrypt0`] / [`arrays::CoseEncrypt0Cbor`] |
//! | `COSE_Encrypt0_Tagged = #6.16(COSE_Encrypt0)` | [`arrays::TaggedCoseEncrypt0`] |
//! | `Enc_structure` | [`arrays::EncStructure`] / [`arrays::EncStructureCbor`] |
//! | `COSE_Mac` | [`arrays::CoseMac`] / [`arrays::CoseMacCbor`] |
//! | `COSE_Mac_Tagged = #6.97(COSE_Mac)` | [`arrays::TaggedCoseMac`] |
//! | `COSE_Mac0` | [`arrays::CoseMac0`] / [`arrays::CoseMac0Cbor`] |
//! | `COSE_Mac0_Tagged = #6.17(COSE_Mac0)` | [`arrays::TaggedCoseMac0`] |
//! | `MAC_structure` | [`arrays::MacStructure`] / [`arrays::MacStructureCbor`] |
//! | `empty_or_serialized_map` | [`choices::EmptyOrSerializedMap`] |
//! | `Sig_structure` context | [`choices::SignatureOrSignature1`] |
//! | `Enc_structure` context | [`choices::EncStructureContext`] |
//! | `MAC_structure` context | [`choices::MacStructureContext`] |
//! | `header_map` / `Generic_Headers` | [`maps::HeaderMap`] / [`maps::HeaderMapCbor`] |
//! | `COSE_Key` | [`maps::CoseKey`] / [`maps::CoseKeyCbor`] |
//! | `COSE_KeySet` | [`maps::CoseKeySet`] |
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(unexpected_cfgs)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod arrays;
pub mod choices;
pub mod maps;
