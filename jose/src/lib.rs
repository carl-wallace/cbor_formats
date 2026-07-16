//! # JOSE — JSON Object Signing and Encryption
//!
//! Implements JWS (RFC 7515), JWK (RFC 7517), JOSE Header structures, and JWE (RFC 7516)
//! compact-form decrypt (initial scope: A256KW + A256GCM).
//! Algorithm-agnostic: signing, verification, key-wrap, and AEAD are delegated to
//! `cose_crypto` trait implementations.
//!
//! ## Not in scope
//!
//! JWT claims validation, SD-JWT. JWE encrypt and JWE JSON serializations not yet implemented.

#![forbid(unsafe_code)]

extern crate alloc;

pub mod error;
pub mod header;
pub mod jwe;
pub mod jwk;
pub mod jws;

pub use error::JoseError;
pub use header::JoseHeader;
pub use jwe::{decrypt_compact as jwe_decrypt_compact, Jwe};
pub use jwk::{Jwk, JwkSet};
pub use jws::{
    verify_compact, verify_compact_detached, verify_flat_json, Jws, JwsBuilder, JwsFlatJson,
    JwsJson, JwsJsonSignature,
};
