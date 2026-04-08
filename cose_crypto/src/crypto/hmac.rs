//! HMAC implementations (HS256/64, HS256, HS384, HS512).

use alloc::string::ToString;
use alloc::vec::Vec;

use ::hmac::{Hmac, Mac};
use cose::maps::CoseKeyCbor;
use sha2::{Sha256, Sha384, Sha512};

use crate::algorithm::CoseAlgorithm;
use crate::error::CoseCryptoError;
use crate::keys::{self, ParsedCoseKey};

use super::CoseMacAlgorithm;

/// HMAC-SHA-256 key, used for both HS256 (full 32-byte tag) and HS256/64 (truncated 8-byte tag).
pub struct HmacSha256Key {
    key_bytes: Vec<u8>,
    algorithm: CoseAlgorithm,
}

impl HmacSha256Key {
    /// Create from raw key bytes with the specified algorithm (HS256 or HS256/64).
    pub fn from_bytes(key: &[u8], algorithm: CoseAlgorithm) -> Result<Self, CoseCryptoError> {
        match algorithm {
            CoseAlgorithm::Hs256 | CoseAlgorithm::Hs256_64 => Ok(Self {
                key_bytes: key.to_vec(),
                algorithm,
            }),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected HS256 or HS256/64 algorithm".to_string(),
            )),
        }
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(
        cose_key: &CoseKeyCbor,
        algorithm: CoseAlgorithm,
    ) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::Symmetric { k } => Self::from_bytes(&k, algorithm),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected symmetric key".to_string(),
            )),
        }
    }
}

impl CoseMacAlgorithm for HmacSha256Key {
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key_bytes)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        mac.update(data);
        let result = mac.finalize().into_bytes().to_vec();
        match self.algorithm {
            CoseAlgorithm::Hs256_64 => Ok(result[..8].to_vec()),
            _ => Ok(result),
        }
    }

    fn verify(&self, data: &[u8], tag: &[u8]) -> Result<(), CoseCryptoError> {
        let computed = self.compute(data)?;
        let expected_len = self.algorithm.tag_size().unwrap_or(computed.len());
        if tag.len() != expected_len || !constant_time_eq(&computed[..expected_len], tag) {
            return Err(CoseCryptoError::MacVerificationFailed);
        }
        Ok(())
    }

    fn algorithm(&self) -> CoseAlgorithm {
        self.algorithm
    }
}

/// HMAC-SHA-384 key.
pub struct HmacSha384Key {
    key_bytes: Vec<u8>,
}

impl HmacSha384Key {
    /// Create from raw key bytes.
    pub fn from_bytes(key: &[u8]) -> Self {
        Self {
            key_bytes: key.to_vec(),
        }
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::Symmetric { k } => Ok(Self::from_bytes(&k)),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected symmetric key".to_string(),
            )),
        }
    }
}

impl CoseMacAlgorithm for HmacSha384Key {
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        let mut mac = Hmac::<Sha384>::new_from_slice(&self.key_bytes)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn verify(&self, data: &[u8], tag: &[u8]) -> Result<(), CoseCryptoError> {
        let computed = self.compute(data)?;
        if tag.len() != computed.len() || !constant_time_eq(&computed, tag) {
            return Err(CoseCryptoError::MacVerificationFailed);
        }
        Ok(())
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Hs384
    }
}

/// HMAC-SHA-512 key.
pub struct HmacSha512Key {
    key_bytes: Vec<u8>,
}

impl HmacSha512Key {
    /// Create from raw key bytes.
    pub fn from_bytes(key: &[u8]) -> Self {
        Self {
            key_bytes: key.to_vec(),
        }
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::Symmetric { k } => Ok(Self::from_bytes(&k)),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected symmetric key".to_string(),
            )),
        }
    }
}

impl CoseMacAlgorithm for HmacSha512Key {
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        let mut mac = Hmac::<Sha512>::new_from_slice(&self.key_bytes)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn verify(&self, data: &[u8], tag: &[u8]) -> Result<(), CoseCryptoError> {
        let computed = self.compute(data)?;
        if tag.len() != computed.len() || !constant_time_eq(&computed, tag) {
            return Err(CoseCryptoError::MacVerificationFailed);
        }
        Ok(())
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Hs512
    }
}

/// Constant-time byte comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
