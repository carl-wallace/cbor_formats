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
//! ## Module Overview
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`algorithm`] | [`CoseAlgorithm`](algorithm::CoseAlgorithm) enum (ES256, ES384, EdDSA, HMAC, AES-GCM variants) |
//! | [`crypto`] | Traits: [`CoseSigner`](crypto::CoseSigner), [`CoseVerifier`](crypto::CoseVerifier), [`CoseMacAlgorithm`](crypto::CoseMacAlgorithm), [`CoseAead`](crypto::CoseAead); impls for ECDSA, EdDSA, HMAC, AES-GCM |
//! | [`sign`] | [`CoseSign1Builder`](sign::CoseSign1Builder), [`CoseSignBuilder`](sign::CoseSignBuilder) |
//! | [`mac`] | [`CoseMac0Builder`](mac::CoseMac0Builder), [`CoseMacBuilder`](mac::CoseMacBuilder) |
//! | [`encrypt`] | [`CoseEncrypt0Builder`](encrypt::CoseEncrypt0Builder) |
//! | [`keys`] | [`ParsedCoseKey`](keys::ParsedCoseKey) — extract typed key material from `CoseKeyCbor` |
//! | [`jwk`] | JWK-to-COSE key parsing (EC P-256, P-384, OKP Ed25519) |
//! | [`helpers`] | Protected header serialization, algorithm extraction, IV handling |
//! | [`error`] | [`CoseCryptoError`](error::CoseCryptoError) |

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
