//! AES-GCM AEAD implementations (A128GCM, A192GCM, A256GCM).
#![allow(deprecated)]

use alloc::string::ToString;
use alloc::vec::Vec;

use aes_gcm::aead::Aead;
use aes_gcm::aead::KeyInit;
use aes_gcm::aead::Payload;
use aes_gcm::{Aes128Gcm, Aes256Gcm, Nonce};
use cose::maps::CoseKeyCbor;

use crate::algorithm::CoseAlgorithm;
use crate::error::CoseCryptoError;
use crate::keys::{self, ParsedCoseKey};

use super::CoseAead;

/// AES-GCM key supporting 128-bit and 256-bit key sizes.
pub enum AesGcmKey {
    /// AES-128-GCM
    Aes128(Box<Aes128Gcm>),
    /// AES-256-GCM
    Aes256(Box<Aes256Gcm>),
}

impl AesGcmKey {
    /// Create from raw key bytes. Key length determines algorithm (16=A128GCM, 32=A256GCM).
    pub fn from_bytes(key: &[u8]) -> Result<Self, CoseCryptoError> {
        match key.len() {
            16 => {
                let cipher = Aes128Gcm::new_from_slice(key)
                    .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
                Ok(Self::Aes128(Box::new(cipher)))
            }
            32 => {
                let cipher = Aes256Gcm::new_from_slice(key)
                    .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
                Ok(Self::Aes256(Box::new(cipher)))
            }
            other => Err(CoseCryptoError::InvalidKey(alloc::format!(
                "AES-GCM key must be 16 or 32 bytes, got {other}"
            ))),
        }
    }

    /// Create from a COSE key structure.
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::Symmetric { k } => Self::from_bytes(&k),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected symmetric key".to_string(),
            )),
        }
    }
}

impl CoseAead for AesGcmKey {
    fn encrypt(
        &self,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CoseCryptoError> {
        let nonce = Nonce::from_slice(nonce);
        let payload = Payload {
            msg: plaintext,
            aad,
        };
        match self {
            Self::Aes128(cipher) => cipher
                .encrypt(nonce, payload)
                .map_err(|_| CoseCryptoError::DecryptionFailed),
            Self::Aes256(cipher) => cipher
                .encrypt(nonce, payload)
                .map_err(|_| CoseCryptoError::DecryptionFailed),
        }
    }

    fn decrypt(
        &self,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CoseCryptoError> {
        let nonce = Nonce::from_slice(nonce);
        let payload = Payload {
            msg: ciphertext,
            aad,
        };
        match self {
            Self::Aes128(cipher) => cipher
                .decrypt(nonce, payload)
                .map_err(|_| CoseCryptoError::DecryptionFailed),
            Self::Aes256(cipher) => cipher
                .decrypt(nonce, payload)
                .map_err(|_| CoseCryptoError::DecryptionFailed),
        }
    }

    fn algorithm(&self) -> CoseAlgorithm {
        match self {
            Self::Aes128(_) => CoseAlgorithm::A128Gcm,
            Self::Aes256(_) => CoseAlgorithm::A256Gcm,
        }
    }
}
