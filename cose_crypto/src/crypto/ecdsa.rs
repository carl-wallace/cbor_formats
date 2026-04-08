//! ECDSA signing and verification (ES256, ES384).

use alloc::string::ToString;
use alloc::vec::Vec;

use ::ecdsa::signature::Signer;
use ::ecdsa::signature::Verifier;
use cose::maps::CoseKeyCbor;

use crate::algorithm::CoseAlgorithm;
use crate::error::CoseCryptoError;
use crate::keys::{self, ParsedCoseKey};

use super::{CoseSigner, CoseVerifier};

// ── ES256 (P-256 / NIST P-256) ──

/// ES256 signer using a P-256 private key.
pub struct Es256Signer {
    key: p256::ecdsa::SigningKey,
}

impl Es256Signer {
    /// Create from raw private key scalar bytes (32 bytes).
    pub fn from_bytes(d: &[u8]) -> Result<Self, CoseCryptoError> {
        let key = p256::ecdsa::SigningKey::from_bytes(d.into())
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        Ok(Self { key })
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::EcPrivate { crv, d, .. } if crv == keys::P256 => Self::from_bytes(&d),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected EC2 P-256 private key".to_string(),
            )),
        }
    }
}

impl CoseSigner for Es256Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        let sig: p256::ecdsa::Signature = self.key.sign(data);
        Ok(sig.to_bytes().to_vec())
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Es256
    }
}

/// ES256 verifier using a P-256 public key.
pub struct Es256Verifier {
    key: p256::ecdsa::VerifyingKey,
}

impl Es256Verifier {
    /// Create from uncompressed SEC1 x and y coordinate bytes (32 bytes each).
    pub fn from_xy(x: &[u8], y: &[u8]) -> Result<Self, CoseCryptoError> {
        let mut uncompressed = Vec::with_capacity(1 + x.len() + y.len());
        uncompressed.push(0x04);
        uncompressed.extend_from_slice(x);
        uncompressed.extend_from_slice(y);
        let point = p256::EncodedPoint::from_bytes(&uncompressed)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        let key = p256::ecdsa::VerifyingKey::from_encoded_point(&point)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        Ok(Self { key })
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::EcPublic { crv, x, y } if crv == keys::P256 => Self::from_xy(&x, &y),
            ParsedCoseKey::EcPrivate { crv, x, y, .. } if crv == keys::P256 => {
                Self::from_xy(&x, &y)
            }
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected EC2 P-256 key".to_string(),
            )),
        }
    }
}

impl CoseVerifier for Es256Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), CoseCryptoError> {
        let sig = p256::ecdsa::Signature::from_bytes(signature.into())
            .map_err(|_| CoseCryptoError::VerificationFailed)?;
        self.key
            .verify(data, &sig)
            .map_err(|_| CoseCryptoError::VerificationFailed)
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Es256
    }
}

// ── ES384 (P-384 / NIST P-384) ──

/// ES384 signer using a P-384 private key.
pub struct Es384Signer {
    key: p384::ecdsa::SigningKey,
}

impl Es384Signer {
    /// Create from raw private key scalar bytes (48 bytes).
    pub fn from_bytes(d: &[u8]) -> Result<Self, CoseCryptoError> {
        let key = p384::ecdsa::SigningKey::from_bytes(d.into())
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        Ok(Self { key })
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::EcPrivate { crv, d, .. } if crv == keys::P384 => Self::from_bytes(&d),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected EC2 P-384 private key".to_string(),
            )),
        }
    }
}

impl CoseSigner for Es384Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        let sig: p384::ecdsa::Signature = self.key.sign(data);
        Ok(sig.to_bytes().to_vec())
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Es384
    }
}

/// ES384 verifier using a P-384 public key.
pub struct Es384Verifier {
    key: p384::ecdsa::VerifyingKey,
}

impl Es384Verifier {
    /// Create from uncompressed SEC1 x and y coordinate bytes (48 bytes each).
    pub fn from_xy(x: &[u8], y: &[u8]) -> Result<Self, CoseCryptoError> {
        let mut uncompressed = Vec::with_capacity(1 + x.len() + y.len());
        uncompressed.push(0x04);
        uncompressed.extend_from_slice(x);
        uncompressed.extend_from_slice(y);
        let point = p384::EncodedPoint::from_bytes(&uncompressed)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        let key = p384::ecdsa::VerifyingKey::from_encoded_point(&point)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        Ok(Self { key })
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::EcPublic { crv, x, y } if crv == keys::P384 => Self::from_xy(&x, &y),
            ParsedCoseKey::EcPrivate { crv, x, y, .. } if crv == keys::P384 => {
                Self::from_xy(&x, &y)
            }
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected EC2 P-384 key".to_string(),
            )),
        }
    }
}

impl CoseVerifier for Es384Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), CoseCryptoError> {
        let sig = p384::ecdsa::Signature::from_bytes(signature.into())
            .map_err(|_| CoseCryptoError::VerificationFailed)?;
        self.key
            .verify(data, &sig)
            .map_err(|_| CoseCryptoError::VerificationFailed)
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Es384
    }
}
