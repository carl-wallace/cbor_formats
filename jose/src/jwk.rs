//! JSON Web Key (JWK) and JWK Set (RFC 7517).
//!
//! `Jwk` is a newtype over `Map<String, Value>` to support all key types
//! (EC, OKP, AKP, symmetric) without a fixed field set.

use alloc::{boxed::Box, string::String, vec::Vec};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use cose_crypto::crypto::{CoseSigner, CoseVerifier};

use crate::error::JoseError;

/// A JSON Web Key — newtype over a JSON object.
///
/// JWK parameter sets vary by key type: EC uses `crv`/`x`/`y`/`d`,
/// OKP uses `crv`/`x`/`d`, AKP (ML-DSA) uses `pub`/`priv`, and
/// symmetric uses `k`. This newtype covers all current and future
/// key types without struct changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Jwk(pub Map<String, Value>);

impl Jwk {
    /// Create an empty JWK.
    pub fn new() -> Self {
        Jwk(Map::new())
    }

    /// Create a JWK from a JSON byte slice.
    pub fn from_json(json: &[u8]) -> Result<Self, JoseError> {
        serde_json::from_slice(json)
            .map_err(|e| JoseError::InvalidHeader(format!("JWK parse error: {e}")))
    }

    // -- Convenience getters for common parameters --

    fn get_str(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(Value::as_str)
    }

    /// Key Type (`kty`).
    pub fn kty(&self) -> Option<&str> {
        self.get_str("kty")
    }

    /// Algorithm (`alg`).
    pub fn alg(&self) -> Option<&str> {
        self.get_str("alg")
    }

    /// Key ID (`kid`).
    pub fn kid(&self) -> Option<&str> {
        self.get_str("kid")
    }

    /// Curve (`crv`) — EC and OKP key types.
    pub fn crv(&self) -> Option<&str> {
        self.get_str("crv")
    }

    /// Public Key Use (`use`).
    pub fn use_(&self) -> Option<&str> {
        self.get_str("use")
    }

    /// Key Operations (`key_ops`).
    pub fn key_ops(&self) -> Option<Vec<&str>> {
        self.0
            .get("key_ops")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_str).collect())
    }

    /// X.509 Certificate Chain (`x5c`).
    pub fn x5c(&self) -> Option<Vec<&str>> {
        self.0
            .get("x5c")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_str).collect())
    }

    /// X.509 Certificate SHA-256 Thumbprint (`x5t#S256`).
    pub fn x5t_s256(&self) -> Option<&str> {
        self.get_str("x5t#S256")
    }

    // -- Convenience setters --

    /// Set a string parameter.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.0.insert(key.into(), value.into());
        self
    }

    // -- Crypto dispatch --

    /// Serialize this JWK to JSON bytes.
    pub fn to_json(&self) -> Result<Vec<u8>, JoseError> {
        serde_json::to_vec(&self.0)
            .map_err(|e| JoseError::InvalidEncoding(format!("JWK serialize error: {e}")))
    }

    /// Create a [`CoseSigner`] from this JWK.
    ///
    /// Delegates to `cose_crypto::jwk::signer_from_jwk()`.
    pub fn to_signer(&self) -> Result<Box<dyn CoseSigner>, JoseError> {
        let json = self.to_json()?;
        cose_crypto::jwk::signer_from_jwk(&json).map_err(JoseError::from)
    }

    /// Create a [`CoseVerifier`] from this JWK.
    ///
    /// Delegates to `cose_crypto::jwk::verifier_from_jwk()`.
    pub fn to_verifier(&self) -> Result<Box<dyn CoseVerifier>, JoseError> {
        let json = self.to_json()?;
        cose_crypto::jwk::verifier_from_jwk(&json).map_err(JoseError::from)
    }
}

impl Default for Jwk {
    fn default() -> Self {
        Self::new()
    }
}

/// A JWK Set (RFC 7517 §5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}

impl JwkSet {
    /// Create an empty JWK Set.
    pub fn new() -> Self {
        JwkSet { keys: Vec::new() }
    }

    /// Find a key by `kid`.
    pub fn find_by_kid(&self, kid: &str) -> Option<&Jwk> {
        self.keys.iter().find(|k| k.kid() == Some(kid))
    }
}

impl Default for JwkSet {
    fn default() -> Self {
        Self::new()
    }
}
