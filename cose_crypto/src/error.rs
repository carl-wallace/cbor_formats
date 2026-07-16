//! Error types for COSE cryptographic operations.

use alloc::string::String;
use core::fmt;

use crate::algorithm::CoseAlgorithm;

/// Errors that can occur during COSE cryptographic operations.
#[derive(Debug)]
pub enum CoseCryptoError {
    /// The algorithm is not supported.
    UnsupportedAlgorithm(i64),
    /// No algorithm was specified in the protected headers.
    MissingAlgorithm,
    /// The algorithm in the protected header does not match the key/signer algorithm.
    AlgorithmMismatch {
        /// Algorithm from the protected header.
        header: CoseAlgorithm,
        /// Algorithm from the key/signer/MAC/AEAD.
        key: CoseAlgorithm,
    },
    /// The key type does not match the algorithm.
    KeyMismatch(String),
    /// The key material is invalid.
    InvalidKey(String),
    /// Signature verification failed.
    VerificationFailed,
    /// MAC verification failed.
    MacVerificationFailed,
    /// Encryption failed.
    EncryptionFailed,
    /// Decryption failed.
    DecryptionFailed,
    /// CBOR encoding/decoding error.
    CborError(String),
    /// A required field is missing.
    MissingField(String),
    /// The protected header bytes are invalid.
    InvalidProtectedHeader(String),
}

impl fmt::Display for CoseCryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedAlgorithm(alg) => write!(f, "unsupported algorithm: {alg}"),
            Self::MissingAlgorithm => write!(f, "no algorithm specified in protected headers"),
            Self::AlgorithmMismatch { header, key } => write!(
                f,
                "algorithm mismatch: header specifies {header:?} but key/signer uses {key:?}"
            ),
            Self::KeyMismatch(msg) => write!(f, "key mismatch: {msg}"),
            Self::InvalidKey(msg) => write!(f, "invalid key: {msg}"),
            Self::VerificationFailed => write!(f, "signature verification failed"),
            Self::MacVerificationFailed => write!(f, "MAC verification failed"),
            Self::EncryptionFailed => write!(f, "encryption failed"),
            Self::DecryptionFailed => write!(f, "decryption failed"),
            Self::CborError(msg) => write!(f, "CBOR error: {msg}"),
            Self::MissingField(msg) => write!(f, "missing field: {msg}"),
            Self::InvalidProtectedHeader(msg) => write!(f, "invalid protected header: {msg}"),
        }
    }
}

impl core::error::Error for CoseCryptoError {}
