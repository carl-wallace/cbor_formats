//! AES Key Wrap implementations per RFC 3394 (A128KW and A256KW).
//!
//! Initial scope was A256KW (Google Play Integrity Standard self-managed JWE-decryption path,
//! header `{"alg":"A256KW","enc":"A256GCM"}`). A128KW added 2026-06-10 to enable the RFC 7520
//! §5.8 worked-example test vector. A192KW can be added symmetrically when a consumer needs it.

use alloc::{string::ToString, vec, vec::Vec};

use aes_kw::{KwAes128, KwAes256, cipher::KeyInit};

use cose::maps::CoseKeyCbor;

use super::CoseKeyWrap;
use crate::{
    algorithm::CoseAlgorithm,
    error::CoseCryptoError,
    keys::{self, ParsedCoseKey},
};

const KW_IV_LEN: usize = 8;

/// AES Key Wrap key (128- or 256-bit AES KEK).
///
/// Variants are boxed because `KwAes128` and `KwAes256` hold expanded AES key
/// schedules (~704 / ~960 bytes); inlining them pushes the enum past 976
/// bytes total and creates stack pressure for downstream consumers carrying
/// this type by value through generic chains. The wrap/unwrap path
/// allocates once per session, so the heap indirection is sub-microsecond
/// noise — see the clippy `large_enum_variant` finding 2026-06-11.
pub enum AesKwKey {
    /// AES-128 Key Wrap (A128KW)
    Aes128(Box<KwAes128>),
    /// AES-256 Key Wrap (A256KW)
    Aes256(Box<KwAes256>),
}

impl AesKwKey {
    /// Create from raw key bytes. Length determines algorithm (16 = A128KW, 32 = A256KW).
    pub fn from_bytes(key: &[u8]) -> Result<Self, CoseCryptoError> {
        match key.len() {
            16 => {
                let kek = KwAes128::new_from_slice(key)
                    .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
                Ok(Self::Aes128(Box::new(kek)))
            }
            32 => {
                let kek = KwAes256::new_from_slice(key)
                    .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
                Ok(Self::Aes256(Box::new(kek)))
            }
            other => Err(CoseCryptoError::InvalidKey(alloc::format!(
                "AES-KW key must be 16 (A128KW) or 32 bytes (A256KW), got {other}"
            ))),
        }
    }

    /// Create from a COSE key structure (expects a symmetric key entry).
    pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
        match keys::parse_cose_key(cose_key)? {
            ParsedCoseKey::Symmetric { k } => Self::from_bytes(&k),
            _ => Err(CoseCryptoError::KeyMismatch(
                "expected symmetric key".to_string(),
            )),
        }
    }
}

impl CoseKeyWrap for AesKwKey {
    fn wrap(&self, key_to_wrap: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        if key_to_wrap.is_empty() || key_to_wrap.len() % KW_IV_LEN != 0 {
            return Err(CoseCryptoError::InvalidKey(alloc::format!(
                "AES-KW input must be a non-zero multiple of 8 bytes, got {}",
                key_to_wrap.len()
            )));
        }
        let mut buf = vec![0u8; key_to_wrap.len() + KW_IV_LEN];
        match self {
            Self::Aes128(kek) => {
                kek.wrap_key(key_to_wrap, &mut buf)
                    .map_err(|_| CoseCryptoError::EncryptionFailed)?;
            }
            Self::Aes256(kek) => {
                kek.wrap_key(key_to_wrap, &mut buf)
                    .map_err(|_| CoseCryptoError::EncryptionFailed)?;
            }
        }
        Ok(buf)
    }

    fn unwrap(&self, wrapped: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
        if wrapped.len() <= KW_IV_LEN || wrapped.len() % KW_IV_LEN != 0 {
            return Err(CoseCryptoError::InvalidKey(alloc::format!(
                "AES-KW wrapped input must be a multiple of 8 bytes and larger than 8, got {}",
                wrapped.len()
            )));
        }
        let mut buf = vec![0u8; wrapped.len() - KW_IV_LEN];
        match self {
            Self::Aes128(kek) => {
                kek.unwrap_key(wrapped, &mut buf)
                    .map_err(|_| CoseCryptoError::DecryptionFailed)?;
            }
            Self::Aes256(kek) => {
                kek.unwrap_key(wrapped, &mut buf)
                    .map_err(|_| CoseCryptoError::DecryptionFailed)?;
            }
        }
        Ok(buf)
    }

    fn algorithm(&self) -> CoseAlgorithm {
        match self {
            Self::Aes128(_) => CoseAlgorithm::A128Kw,
            Self::Aes256(_) => CoseAlgorithm::A256Kw,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use hex_literal::hex;

    // RFC 3394 § 4.6 — wrap 256 bits of key data with a 256-bit KEK.
    const RFC_3394_4_6_KEK: [u8; 32] =
        hex!("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F");
    const RFC_3394_4_6_KEY: [u8; 32] =
        hex!("00112233445566778899AABBCCDDEEFF000102030405060708090A0B0C0D0E0F");
    const RFC_3394_4_6_WRAPPED: [u8; 40] =
        hex!("28C9F404C4B810F4CBCCB35CFB87F8263F5786E2D80ED326CBC7F0E71A99F43BFB988B9B7A02DD21");

    #[test]
    fn rfc_3394_4_6_wrap_kat() {
        let kek = AesKwKey::from_bytes(&RFC_3394_4_6_KEK).unwrap();
        let wrapped = kek.wrap(&RFC_3394_4_6_KEY).unwrap();
        assert_eq!(wrapped, RFC_3394_4_6_WRAPPED);
    }

    #[test]
    fn rfc_3394_4_6_unwrap_kat() {
        let kek = AesKwKey::from_bytes(&RFC_3394_4_6_KEK).unwrap();
        let unwrapped = kek.unwrap(&RFC_3394_4_6_WRAPPED).unwrap();
        assert_eq!(unwrapped, RFC_3394_4_6_KEY);
    }

    #[test]
    fn round_trip() {
        let kek = AesKwKey::from_bytes(&[0x42u8; 32]).unwrap();
        let cek = [0x37u8; 32];
        let wrapped = kek.wrap(&cek).unwrap();
        assert_eq!(wrapped.len(), 40);
        let unwrapped = kek.unwrap(&wrapped).unwrap();
        assert_eq!(unwrapped, cek);
    }

    #[test]
    fn unwrap_rejects_tampered_iv() {
        let kek = AesKwKey::from_bytes(&[0x42u8; 32]).unwrap();
        let mut wrapped = kek.wrap(&[0x37u8; 32]).unwrap();
        wrapped[0] ^= 0x01;
        assert!(matches!(
            kek.unwrap(&wrapped),
            Err(CoseCryptoError::DecryptionFailed)
        ));
    }

    #[test]
    fn rejects_wrong_key_length() {
        // 24 bytes = A192KW which we don't support yet; 8 and 64 are nonsense.
        assert!(AesKwKey::from_bytes(&[0u8; 8]).is_err());
        assert!(AesKwKey::from_bytes(&[0u8; 24]).is_err());
        assert!(AesKwKey::from_bytes(&[0u8; 64]).is_err());
    }

    #[test]
    fn aes128_round_trip() {
        let kek = AesKwKey::from_bytes(&[0x42u8; 16]).unwrap();
        assert_eq!(kek.algorithm(), CoseAlgorithm::A128Kw);
        let cek = [0x37u8; 16];
        let wrapped = kek.wrap(&cek).unwrap();
        assert_eq!(wrapped.len(), 24);
        let unwrapped = kek.unwrap(&wrapped).unwrap();
        assert_eq!(unwrapped, cek);
    }
}
