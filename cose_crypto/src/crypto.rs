//! Cryptographic trait definitions and implementations for COSE operations.

use alloc::vec::Vec;

use crate::{algorithm::CoseAlgorithm, error::CoseCryptoError};

pub mod aes_gcm;
pub mod aes_kw;
#[cfg(feature = "ecdsa")]
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

/// A key-wrap algorithm that can wrap and unwrap symmetric keys (RFC 3394 AES Key Wrap).
///
/// Unlike AEAD, key wrap has no nonce and no AAD — the wrapped output is self-contained and
/// includes its own integrity check via the RFC 3394 default IV.
pub trait CoseKeyWrap {
    /// Wrap a symmetric key. Input must be a multiple of 8 bytes (typically 16, 24, or 32).
    fn wrap(&self, key_to_wrap: &[u8]) -> Result<Vec<u8>, CoseCryptoError>;
    /// Unwrap a previously wrapped key. Output is 8 bytes shorter than input.
    fn unwrap(&self, wrapped: &[u8]) -> Result<Vec<u8>, CoseCryptoError>;
    /// The algorithm this key wrap uses.
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
