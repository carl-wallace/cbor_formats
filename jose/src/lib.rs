//! # JOSE — JSON Object Signing and Encryption
//!
//! Implements JWS (RFC 7515), JWK (RFC 7517), and JOSE Header structures.
//! Algorithm-agnostic: signing and verification are delegated to
//! `cose_crypto` signer/verifier implementations.
//!
//! ## Not in scope
//!
//! JWE (RFC 7516), JWT claims validation, SD-JWT.

#![forbid(unsafe_code)]

extern crate alloc;

pub mod error;
pub mod header;
pub mod jwk;
pub mod jws;

pub use error::JoseError;
pub use header::JoseHeader;
pub use jwk::{Jwk, JwkSet};
pub use jws::{
    verify_compact, verify_compact_detached, verify_flat_json, Jws, JwsBuilder, JwsFlatJson,
    JwsJson, JwsJsonSignature,
};
