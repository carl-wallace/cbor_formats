//! EdDSA signing and verification (Ed25519).

use alloc::{string::ToString, vec::Vec};

use ed25519_dalek::{Signer, Verifier};

use cose::maps::CoseKeyCbor;

use super::{CoseSigner, CoseVerifier};
use crate::{
    algorithm::CoseAlgorithm,
    error::CoseCryptoError,
    keys::{self, ParsedCoseKey},
};

/// Ed25519 signer.
pub struct Ed25519Signer {
    key: ed25519_dalek::SigningKey,
}

impl Ed25519Signer {
    /// Create from raw private key bytes (32 bytes).
    pub fn from_bytes(d: &[u8]) -> Result<Self, CoseCryptoError> {
        let bytes: [u8; 32] = d
            .try_into()
            .map_err(|_| CoseCryptoError::InvalidKey("Ed25519 key must be 32 bytes".to_string()))?;
        Ok(Self {
            key: ed25519_dalek::SigningKey::from_bytes(&bytes),
        })
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::OkpPrivate { crv, d, .. } if crv == keys::ED25519 => {
                Self::from_bytes(&d)
            }
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected OKP Ed25519 private key".to_string(),
            )),
        }
    }
}

impl CoseSigner for Ed25519Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        let sig = self.key.sign(data);
        Ok(sig.to_bytes().to_vec())
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Eddsa
    }
}

/// Ed25519 verifier.
pub struct Ed25519Verifier {
    key: ed25519_dalek::VerifyingKey,
}

impl Ed25519Verifier {
    /// Create from raw public key bytes (32 bytes).
    pub fn from_bytes(x: &[u8]) -> Result<Self, CoseCryptoError> {
        let bytes: [u8; 32] = x
            .try_into()
            .map_err(|_| CoseCryptoError::InvalidKey("Ed25519 key must be 32 bytes".to_string()))?;
        let key = ed25519_dalek::VerifyingKey::from_bytes(&bytes)
            .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
        Ok(Self { key })
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::OkpPublic { crv, x } if crv == keys::ED25519 => Self::from_bytes(&x),
            ParsedCoseKey::OkpPrivate { crv, x, .. } if crv == keys::ED25519 => {
                Self::from_bytes(&x)
            }
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected OKP Ed25519 key".to_string(),
            )),
        }
    }
}

impl CoseVerifier for Ed25519Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), CoseCryptoError> {
        let bytes: [u8; 64] = signature
            .try_into()
            .map_err(|_| CoseCryptoError::VerificationFailed)?;
        let sig = ed25519_dalek::Signature::from_bytes(&bytes);
        self.key
            .verify(data, &sig)
            .map_err(|_| CoseCryptoError::VerificationFailed)
    }

    fn algorithm(&self) -> CoseAlgorithm {
        CoseAlgorithm::Eddsa
    }
}
