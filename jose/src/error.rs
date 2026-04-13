//! Error types for JOSE operations.

use alloc::string::String;
use core::fmt;

use cose_crypto::error::CoseCryptoError;

/// Errors that can occur during JOSE operations.
#[derive(Debug)]
pub enum JoseError {
    /// The JOSE header is invalid or missing required fields.
    InvalidHeader(String),
    /// Signature verification failed.
    InvalidSignature,
    /// Base64url encoding/decoding failed.
    InvalidEncoding(String),
    /// A required field is missing.
    MissingField(String),
    /// The algorithm is not supported.
    UnsupportedAlgorithm(String),
    /// An error from the underlying cryptographic operations.
    CryptoError(CoseCryptoError),
}

impl fmt::Display for JoseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JoseError::InvalidHeader(msg) => write!(f, "invalid JOSE header: {msg}"),
            JoseError::InvalidSignature => write!(f, "signature verification failed"),
            JoseError::InvalidEncoding(msg) => write!(f, "encoding error: {msg}"),
            JoseError::MissingField(field) => write!(f, "missing field: {field}"),
            JoseError::UnsupportedAlgorithm(alg) => write!(f, "unsupported algorithm: {alg}"),
            JoseError::CryptoError(e) => write!(f, "crypto error: {e}"),
        }
    }
}

impl std::error::Error for JoseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JoseError::CryptoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<CoseCryptoError> for JoseError {
    fn from(e: CoseCryptoError) -> Self {
        JoseError::CryptoError(e)
    }
}
