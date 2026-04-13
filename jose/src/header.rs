//! JOSE Header (RFC 7515 §4).
//!
//! `JoseHeader` is a newtype over `Map<String, Value>` to support all
//! registered, public, and private header parameters without a fixed field set.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::jwk::Jwk;

/// A JOSE Header — newtype over a JSON object.
///
/// Header parameter sets vary by application and extension specs.
/// A fixed struct cannot cover all current and future parameters,
/// so `JoseHeader` wraps the raw map with typed accessors for the
/// registered parameters defined in RFC 7515 §4.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JoseHeader(pub Map<String, Value>);

impl JoseHeader {
    /// Create a header with just the `alg` parameter set.
    pub fn new(alg: &str) -> Self {
        let mut map = Map::new();
        map.insert("alg".to_string(), Value::String(alg.to_string()));
        JoseHeader(map)
    }

    // -- Convenience getters for RFC 7515 §4.1 registered parameters --

    fn get_str(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(Value::as_str)
    }

    /// Algorithm (`alg`) — RFC 7515 §4.1.1.
    pub fn alg(&self) -> Option<&str> {
        self.get_str("alg")
    }

    /// JWK Set URL (`jku`) — RFC 7515 §4.1.2.
    pub fn jku(&self) -> Option<&str> {
        self.get_str("jku")
    }

    /// JSON Web Key (`jwk`) — RFC 7515 §4.1.3.
    pub fn jwk(&self) -> Option<Jwk> {
        self.0.get("jwk").and_then(|v| {
            if let Value::Object(map) = v {
                Some(Jwk(map.clone()))
            } else {
                None
            }
        })
    }

    /// Key ID (`kid`) — RFC 7515 §4.1.4.
    pub fn kid(&self) -> Option<&str> {
        self.get_str("kid")
    }

    /// X.509 URL (`x5u`) — RFC 7515 §4.1.5.
    pub fn x5u(&self) -> Option<&str> {
        self.get_str("x5u")
    }

    /// X.509 Certificate Chain (`x5c`) — RFC 7515 §4.1.6.
    pub fn x5c(&self) -> Option<Vec<&str>> {
        self.0
            .get("x5c")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_str).collect())
    }

    /// X.509 Certificate SHA-1 Thumbprint (`x5t`) — RFC 7515 §4.1.7.
    pub fn x5t(&self) -> Option<&str> {
        self.get_str("x5t")
    }

    /// X.509 Certificate SHA-256 Thumbprint (`x5t#S256`) — RFC 7515 §4.1.8.
    pub fn x5t_s256(&self) -> Option<&str> {
        self.get_str("x5t#S256")
    }

    /// Type (`typ`) — RFC 7515 §4.1.9.
    pub fn typ(&self) -> Option<&str> {
        self.get_str("typ")
    }

    /// Content Type (`cty`) — RFC 7515 §4.1.10.
    pub fn cty(&self) -> Option<&str> {
        self.get_str("cty")
    }

    /// Critical (`crit`) — RFC 7515 §4.1.11.
    pub fn crit(&self) -> Option<Vec<&str>> {
        self.0
            .get("crit")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(Value::as_str).collect())
    }

    // -- Setters (return &mut Self for chaining) --

    /// Set a parameter by key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.0.insert(key.into(), value.into());
        self
    }

    /// Set the Key ID (`kid`).
    pub fn set_kid(&mut self, kid: &str) -> &mut Self {
        self.set("kid", kid)
    }

    /// Set the Type (`typ`).
    pub fn set_typ(&mut self, typ: &str) -> &mut Self {
        self.set("typ", typ)
    }

    /// Set the Content Type (`cty`).
    pub fn set_cty(&mut self, cty: &str) -> &mut Self {
        self.set("cty", cty)
    }

    /// Set the JSON Web Key (`jwk`).
    pub fn set_jwk(&mut self, jwk: &Jwk) -> &mut Self {
        self.0
            .insert("jwk".to_string(), Value::Object(jwk.0.clone()));
        self
    }
}
