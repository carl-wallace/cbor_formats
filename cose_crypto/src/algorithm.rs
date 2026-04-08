//! COSE algorithm registry types and IANA label constants.

use crate::error::CoseCryptoError;
use common::TextOrInt;

/// COSE algorithms supported by this crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoseAlgorithm {
    /// ECDSA w/ SHA-256 (IANA label -7)
    Es256,
    /// ECDSA w/ SHA-384 (IANA label -35)
    Es384,
    /// EdDSA (IANA label -8)
    Eddsa,
    /// HMAC 256/64 (IANA label 4)
    Hs256_64,
    /// HMAC 256/256 (IANA label 5)
    Hs256,
    /// HMAC 384/384 (IANA label 6)
    Hs384,
    /// HMAC 512/512 (IANA label 7)
    Hs512,
    /// AES-GCM 128-bit key (IANA label 1)
    A128Gcm,
    /// AES-GCM 192-bit key (IANA label 2)
    A192Gcm,
    /// AES-GCM 256-bit key (IANA label 3)
    A256Gcm,
}

impl CoseAlgorithm {
    /// Convert from IANA COSE algorithm label.
    pub fn from_i64(label: i64) -> Result<Self, CoseCryptoError> {
        match label {
            -7 => Ok(Self::Es256),
            -35 => Ok(Self::Es384),
            -8 => Ok(Self::Eddsa),
            4 => Ok(Self::Hs256_64),
            5 => Ok(Self::Hs256),
            6 => Ok(Self::Hs384),
            7 => Ok(Self::Hs512),
            1 => Ok(Self::A128Gcm),
            2 => Ok(Self::A192Gcm),
            3 => Ok(Self::A256Gcm),
            other => Err(CoseCryptoError::UnsupportedAlgorithm(other)),
        }
    }

    /// Convert to IANA COSE algorithm label.
    pub fn to_i64(self) -> i64 {
        match self {
            Self::Es256 => -7,
            Self::Es384 => -35,
            Self::Eddsa => -8,
            Self::Hs256_64 => 4,
            Self::Hs256 => 5,
            Self::Hs384 => 6,
            Self::Hs512 => 7,
            Self::A128Gcm => 1,
            Self::A192Gcm => 2,
            Self::A256Gcm => 3,
        }
    }

    /// Convert from a `TextOrInt` value (as found in `HeaderMap.alg_id`).
    pub fn from_text_or_int(toi: &TextOrInt) -> Result<Self, CoseCryptoError> {
        match toi {
            TextOrInt::Int(i) => Self::from_i64(*i),
            TextOrInt::Text(_) => Err(CoseCryptoError::UnsupportedAlgorithm(0)),
        }
    }

    /// Returns the symmetric key size in bytes, if applicable.
    pub fn key_size(self) -> Option<usize> {
        match self {
            Self::Hs256_64 | Self::Hs256 => Some(32),
            Self::Hs384 => Some(48),
            Self::Hs512 => Some(64),
            Self::A128Gcm => Some(16),
            Self::A192Gcm => Some(24),
            Self::A256Gcm => Some(32),
            _ => None,
        }
    }

    /// Returns the nonce/IV size in bytes, if applicable.
    pub fn nonce_size(self) -> Option<usize> {
        match self {
            Self::A128Gcm | Self::A192Gcm | Self::A256Gcm => Some(12),
            _ => None,
        }
    }

    /// Returns the truncated tag length in bytes for HMAC algorithms.
    pub fn tag_size(self) -> Option<usize> {
        match self {
            Self::Hs256_64 => Some(8),
            Self::Hs256 => Some(32),
            Self::Hs384 => Some(48),
            Self::Hs512 => Some(64),
            _ => None,
        }
    }
}
