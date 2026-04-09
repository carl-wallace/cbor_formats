//! Cryptographic trait definitions and implementations for COSE operations.

use alloc::vec::Vec;

use crate::algorithm::CoseAlgorithm;
use crate::error::CoseCryptoError;

pub mod aes_gcm;
pub mod ecdsa;
pub mod eddsa;
pub mod hmac;
#[cfg(feature = "pqc")]
pub mod ml_dsa;

/// A signer that can produce signatures over data.
pub trait CoseSigner {
    /// Sign the given data and return the signature bytes.
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError>;
    /// The algorithm this signer uses.
    fn algorithm(&self) -> CoseAlgorithm;
}

/// A verifier that can check signatures over data.
pub trait CoseVerifier {
    /// Verify the signature over the given data.
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), CoseCryptoError>;
    /// The algorithm this verifier uses.
    fn algorithm(&self) -> CoseAlgorithm;
}

/// A MAC algorithm that can compute and verify message authentication codes.
pub trait CoseMacAlgorithm {
    /// Compute a MAC tag over the given data.
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError>;
    /// Verify a MAC tag over the given data.
    fn verify(&self, data: &[u8], tag: &[u8]) -> Result<(), CoseCryptoError>;
    /// The algorithm this MAC uses.
    fn algorithm(&self) -> CoseAlgorithm;
}

/// An AEAD algorithm that can encrypt and decrypt data.
pub trait CoseAead {
    /// Encrypt plaintext with the given nonce and additional authenticated data.
    fn encrypt(
        &self,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CoseCryptoError>;
    /// Decrypt ciphertext with the given nonce and additional authenticated data.
    fn decrypt(
        &self,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CoseCryptoError>;
    /// The algorithm this AEAD uses.
    fn algorithm(&self) -> CoseAlgorithm;
}
