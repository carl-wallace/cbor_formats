//! Error types for COSE cryptographic operations.

use alloc::string::String;
use core::fmt;

/// Errors that can occur during COSE cryptographic operations.
#[derive(Debug)]
pub enum CoseCryptoError {
    /// The algorithm is not supported.
    UnsupportedAlgorithm(i64),
    /// No algorithm was specified in the protected headers.
    MissingAlgorithm,
    /// The key type does not match the algorithm.
    KeyMismatch(String),
    /// The key material is invalid.
    InvalidKey(String),
    /// Signature verification failed.
    VerificationFailed,
    /// MAC verification failed.
    MacVerificationFailed,
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
            Self::KeyMismatch(msg) => write!(f, "key mismatch: {msg}"),
            Self::InvalidKey(msg) => write!(f, "invalid key: {msg}"),
            Self::VerificationFailed => write!(f, "signature verification failed"),
            Self::MacVerificationFailed => write!(f, "MAC verification failed"),
            Self::DecryptionFailed => write!(f, "decryption failed"),
            Self::CborError(msg) => write!(f, "CBOR error: {msg}"),
            Self::MissingField(msg) => write!(f, "missing field: {msg}"),
            Self::InvalidProtectedHeader(msg) => write!(f, "invalid protected header: {msg}"),
        }
    }
}

impl std::error::Error for CoseCryptoError {}
