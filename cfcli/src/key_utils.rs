//! Key format auto-detection: JWK (JSON) or COSE Key (CBOR).

use cose_crypto::algorithm::CoseAlgorithm;
use cose_crypto::crypto::{CoseSigner, CoseVerifier};
use cose_crypto::error::CoseCryptoError;
use cose_crypto::jwk::{algorithm_from_jwk, signer_from_jwk, verifier_from_jwk};
use cose_crypto::keys::{algorithm_from_cose_key, signer_from_cose_key, verifier_from_cose_key};

/// Try to deserialize `key_bytes` as a COSE Key (CBOR). Returns `None` if
/// the bytes don't look like valid CBOR for a `CoseKeyCbor`.
fn try_parse_cose_key(key_bytes: &[u8]) -> Option<cose::maps::CoseKeyCbor> {
    ciborium::de::from_reader(key_bytes).ok()
}

/// Determine the algorithm from key file bytes (JWK or COSE Key).
pub fn algorithm_from_key(key_bytes: &[u8]) -> Result<CoseAlgorithm, CoseCryptoError> {
    if let Ok(alg) = algorithm_from_jwk(key_bytes) {
        return Ok(alg);
    }
    if let Some(cose_key) = try_parse_cose_key(key_bytes) {
        return algorithm_from_cose_key(&cose_key);
    }
    Err(CoseCryptoError::InvalidKey(
        "key file is neither valid JWK (JSON) nor COSE Key (CBOR)".to_string(),
    ))
}

/// Create a signer from key file bytes (JWK or COSE Key).
pub fn signer_from_key(key_bytes: &[u8]) -> Result<Box<dyn CoseSigner>, CoseCryptoError> {
    if let Ok(signer) = signer_from_jwk(key_bytes) {
        return Ok(signer);
    }
    if let Some(cose_key) = try_parse_cose_key(key_bytes) {
        return signer_from_cose_key(&cose_key);
    }
    Err(CoseCryptoError::InvalidKey(
        "key file is neither valid JWK (JSON) nor COSE Key (CBOR)".to_string(),
    ))
}

/// Create a verifier from key file bytes (JWK or COSE Key).
pub fn verifier_from_key(key_bytes: &[u8]) -> Result<Box<dyn CoseVerifier>, CoseCryptoError> {
    if let Ok(verifier) = verifier_from_jwk(key_bytes) {
        return Ok(verifier);
    }
    if let Some(cose_key) = try_parse_cose_key(key_bytes) {
        return verifier_from_cose_key(&cose_key);
    }
    Err(CoseCryptoError::InvalidKey(
        "key file is neither valid JWK (JSON) nor COSE Key (CBOR)".to_string(),
    ))
}
