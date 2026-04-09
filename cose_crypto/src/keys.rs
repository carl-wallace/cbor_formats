//! COSE key parsing — extracts key material from `CoseKeyCbor` structures.

use alloc::string::ToString;
use alloc::vec::Vec;
use ciborium::value::Value;
use cose::maps::CoseKeyCbor;

use crate::error::CoseCryptoError;

/// COSE key type values (kty parameter, label 1).
const KTY_OKP: i64 = 1;
const KTY_EC2: i64 = 2;
const KTY_SYMMETRIC: i64 = 4;
/// AKP key type (kty=7) for algorithm key pairs (ML-DSA, etc.)
#[cfg(feature = "pqc")]
const KTY_AKP: i64 = 7;

/// EC2/OKP curve values (crv parameter, label -1).
const CRV_P256: i64 = 1;
const CRV_P384: i64 = 2;
const CRV_ED25519: i64 = 6;

/// Parsed COSE key material.
#[derive(Debug)]
pub enum ParsedCoseKey {
    /// EC private key: (curve, x, y, d)
    EcPrivate {
        /// Curve identifier
        crv: i64,
        /// X coordinate
        x: Vec<u8>,
        /// Y coordinate
        y: Vec<u8>,
        /// Private key scalar
        d: Vec<u8>,
    },
    /// EC public key: (curve, x, y)
    EcPublic {
        /// Curve identifier
        crv: i64,
        /// X coordinate
        x: Vec<u8>,
        /// Y coordinate
        y: Vec<u8>,
    },
    /// OKP private key: (curve, x, d)
    OkpPrivate {
        /// Curve identifier
        crv: i64,
        /// Public key bytes
        x: Vec<u8>,
        /// Private key bytes
        d: Vec<u8>,
    },
    /// OKP public key: (curve, x)
    OkpPublic {
        /// Curve identifier
        crv: i64,
        /// Public key bytes
        x: Vec<u8>,
    },
    /// Symmetric key: (k)
    Symmetric {
        /// Key bytes
        k: Vec<u8>,
    },
    /// AKP private key: (alg, pub, priv)
    #[cfg(feature = "pqc")]
    AkpPrivate {
        /// Algorithm identifier (required for AKP keys)
        alg: i64,
        /// Public key bytes (label -1)
        pub_key: Vec<u8>,
        /// Private key bytes (label -2)
        priv_key: Vec<u8>,
    },
    /// AKP public key: (alg, pub)
    #[cfg(feature = "pqc")]
    AkpPublic {
        /// Algorithm identifier (required for AKP keys)
        alg: i64,
        /// Public key bytes (label -1)
        pub_key: Vec<u8>,
    },
}

/// Extract a parameter value from the `other` field of a CoseKeyCbor by integer label.
fn get_param(key: &CoseKeyCbor, label: i64) -> Option<Value> {
    key.other.as_ref()?.iter().find_map(|t| {
        let key_int: i64 = match &t.key {
            Value::Integer(i) => (*i).try_into().ok()?,
            _ => return None,
        };
        if key_int == label {
            Some(t.value.clone())
        } else {
            None
        }
    })
}

/// Extract bytes from a CBOR Value.
fn value_to_bytes(v: &Value) -> Result<Vec<u8>, CoseCryptoError> {
    v.as_bytes()
        .cloned()
        .ok_or_else(|| CoseCryptoError::InvalidKey("expected bytes parameter".to_string()))
}

/// Extract an integer from a CBOR Value.
fn value_to_i64(v: &Value) -> Result<i64, CoseCryptoError> {
    v.as_integer()
        .and_then(|i| i64::try_from(i).ok())
        .ok_or_else(|| CoseCryptoError::InvalidKey("expected integer parameter".to_string()))
}

/// Extract the key type (kty) from a CoseKeyCbor.
fn get_kty(key: &CoseKeyCbor) -> Result<i64, CoseCryptoError> {
    match &key.kty {
        Some(common::TextOrInt::Int(i)) => Ok(*i),
        Some(common::TextOrInt::Text(_)) => Err(CoseCryptoError::InvalidKey(
            "text kty not supported".to_string(),
        )),
        None => Err(CoseCryptoError::InvalidKey("missing kty".to_string())),
    }
}

/// Parse a `CoseKeyCbor` into structured key material.
pub fn parse_cose_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let kty = get_kty(key)?;
    match kty {
        KTY_EC2 => parse_ec2_key(key),
        KTY_OKP => parse_okp_key(key),
        KTY_SYMMETRIC => parse_symmetric_key(key),
        #[cfg(feature = "pqc")]
        KTY_AKP => parse_akp_key(key),
        _ => Err(CoseCryptoError::InvalidKey(alloc::format!(
            "unsupported kty: {kty}"
        ))),
    }
}

fn parse_ec2_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let crv_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing crv (-1)".to_string()))?;
    let crv = value_to_i64(&crv_val)?;

    let x_val = get_param(key, -2)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing x (-2)".to_string()))?;
    let x = value_to_bytes(&x_val)?;

    let y_val = get_param(key, -3)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing y (-3)".to_string()))?;
    let y = value_to_bytes(&y_val)?;

    match get_param(key, -4) {
        Some(d_val) => {
            let d = value_to_bytes(&d_val)?;
            Ok(ParsedCoseKey::EcPrivate { crv, x, y, d })
        }
        None => Ok(ParsedCoseKey::EcPublic { crv, x, y }),
    }
}

fn parse_okp_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let crv_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing crv (-1)".to_string()))?;
    let crv = value_to_i64(&crv_val)?;

    let x_val = get_param(key, -2)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing x (-2)".to_string()))?;
    let x = value_to_bytes(&x_val)?;

    match get_param(key, -4) {
        Some(d_val) => {
            let d = value_to_bytes(&d_val)?;
            Ok(ParsedCoseKey::OkpPrivate { crv, x, d })
        }
        None => Ok(ParsedCoseKey::OkpPublic { crv, x }),
    }
}

fn parse_symmetric_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let k_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing k (-1)".to_string()))?;
    let k = value_to_bytes(&k_val)?;
    Ok(ParsedCoseKey::Symmetric { k })
}

#[cfg(feature = "pqc")]
fn parse_akp_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    // AKP keys require the alg parameter (label 3) to distinguish variants
    let alg = match &key.alg {
        Some(common::TextOrInt::Int(i)) => *i,
        _ => {
            return Err(CoseCryptoError::InvalidKey(
                "AKP keys require alg parameter".to_string(),
            ));
        }
    };

    let pub_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing pub (-1)".to_string()))?;
    let pub_key = value_to_bytes(&pub_val)?;

    match get_param(key, -2) {
        Some(priv_val) => {
            let priv_key = value_to_bytes(&priv_val)?;
            Ok(ParsedCoseKey::AkpPrivate {
                alg,
                pub_key,
                priv_key,
            })
        }
        None => Ok(ParsedCoseKey::AkpPublic { alg, pub_key }),
    }
}

/// Returns the curve identifier for the given CoseKeyCbor, if it's an EC2 or OKP key.
pub fn get_crv(key: &CoseKeyCbor) -> Result<i64, CoseCryptoError> {
    let crv_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing crv (-1)".to_string()))?;
    value_to_i64(&crv_val)
}

/// Re-export curve constants for use by other modules.
pub use self::constants::*;
mod constants {
    /// COSE curve identifier for P-256
    pub const P256: i64 = super::CRV_P256;
    /// COSE curve identifier for P-384
    pub const P384: i64 = super::CRV_P384;
    /// COSE curve identifier for Ed25519
    pub const ED25519: i64 = super::CRV_ED25519;
}
