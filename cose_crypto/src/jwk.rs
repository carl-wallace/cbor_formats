//! JWK (JSON Web Key) parsing for COSE signers and verifiers.
//!
//! Supports EC keys (P-256, P-384) and OKP keys (Ed25519).

use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

use serde::Deserialize;

use base64ct::{Base64UrlUnpadded, Encoding};
use zeroize::Zeroizing;

#[cfg(feature = "ecdsa")]
use crate::crypto::ecdsa::{Es256Signer, Es256Verifier, Es384Signer, Es384Verifier};
use crate::{
    algorithm::CoseAlgorithm,
    crypto::eddsa::{Ed25519Signer, Ed25519Verifier},
    crypto::{CoseSigner, CoseVerifier},
    error::CoseCryptoError,
};

/// Minimal JWK structure for EC and OKP key types.
#[derive(Debug, Deserialize)]
struct Jwk {
    kty: String,
    #[serde(default)]
    crv: Option<String>,
    #[serde(default)]
    x: Option<String>,
    #[serde(default)]
    y: Option<String>,
    #[serde(default)]
    d: Option<String>,
}

/// Decode a base64url-unpadded string to bytes.
fn b64url_decode(s: &str) -> Result<Vec<u8>, CoseCryptoError> {
    Base64UrlUnpadded::decode_vec(s)
        .map_err(|e| CoseCryptoError::InvalidKey(format!("base64url decode error: {e}")))
}

/// Determine the COSE algorithm from a JWK.
pub fn algorithm_from_jwk(json: &[u8]) -> Result<CoseAlgorithm, CoseCryptoError> {
    let jwk: Jwk = serde_json::from_slice(json)
        .map_err(|e| CoseCryptoError::InvalidKey(format!("JWK parse error: {e}")))?;

    match jwk.kty.as_str() {
        "EC" => match jwk.crv.as_deref() {
            Some("P-256") => Ok(CoseAlgorithm::Es256),
            Some("P-384") => Ok(CoseAlgorithm::Es384),
            Some(other) => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported EC curve: {other}"
            ))),
            None => Err(CoseCryptoError::InvalidKey("missing crv".to_string())),
        },
        "OKP" => match jwk.crv.as_deref() {
            Some("Ed25519") => Ok(CoseAlgorithm::Eddsa),
            Some(other) => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported OKP curve: {other}"
            ))),
            None => Err(CoseCryptoError::InvalidKey("missing crv".to_string())),
        },
        other => Err(CoseCryptoError::InvalidKey(format!(
            "unsupported kty: {other}"
        ))),
    }
}

/// Create a [`CoseSigner`] from JWK JSON bytes.
///
/// The JWK must contain a private key (`d` field).
#[cfg_attr(not(feature = "ecdsa"), allow(unused_variables))]
pub fn signer_from_jwk(json: &[u8]) -> Result<Box<dyn CoseSigner>, CoseCryptoError> {
    let jwk: Jwk = serde_json::from_slice(json)
        .map_err(|e| CoseCryptoError::InvalidKey(format!("JWK parse error: {e}")))?;

    let d = jwk
        .d
        .as_deref()
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing private key (d)".to_string()))?;
    let d_bytes = Zeroizing::new(b64url_decode(d)?);

    match jwk.kty.as_str() {
        "EC" => match jwk.crv.as_deref() {
            #[cfg(feature = "ecdsa")]
            Some("P-256") => Ok(Box::new(Es256Signer::from_bytes(&d_bytes)?)),
            #[cfg(feature = "ecdsa")]
            Some("P-384") => Ok(Box::new(Es384Signer::from_bytes(&d_bytes)?)),
            Some(other) => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported EC curve: {other}"
            ))),
            None => Err(CoseCryptoError::InvalidKey("missing crv".to_string())),
        },
        "OKP" => match jwk.crv.as_deref() {
            Some("Ed25519") => Ok(Box::new(Ed25519Signer::from_bytes(&d_bytes)?)),
            Some(other) => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported OKP curve: {other}"
            ))),
            None => Err(CoseCryptoError::InvalidKey("missing crv".to_string())),
        },
        other => Err(CoseCryptoError::InvalidKey(format!(
            "unsupported kty: {other}"
        ))),
    }
}

/// Create a [`CoseVerifier`] from JWK JSON bytes.
///
/// Uses the public key components (`x`, `y` for EC; `x` for OKP).
/// If a private key is present, the public key is derived from it for EC keys,
/// or the `x` component is used for OKP keys.
#[cfg_attr(not(feature = "ecdsa"), allow(unused_variables))]
pub fn verifier_from_jwk(json: &[u8]) -> Result<Box<dyn CoseVerifier>, CoseCryptoError> {
    let jwk: Jwk = serde_json::from_slice(json)
        .map_err(|e| CoseCryptoError::InvalidKey(format!("JWK parse error: {e}")))?;

    match jwk.kty.as_str() {
        "EC" => {
            let x = jwk
                .x
                .as_deref()
                .ok_or_else(|| CoseCryptoError::InvalidKey("missing x".to_string()))?;
            let y = jwk
                .y
                .as_deref()
                .ok_or_else(|| CoseCryptoError::InvalidKey("missing y".to_string()))?;
            let x_bytes = b64url_decode(x)?;
            let y_bytes = b64url_decode(y)?;

            match jwk.crv.as_deref() {
                #[cfg(feature = "ecdsa")]
                Some("P-256") => Ok(Box::new(Es256Verifier::from_xy(&x_bytes, &y_bytes)?)),
                #[cfg(feature = "ecdsa")]
                Some("P-384") => Ok(Box::new(Es384Verifier::from_xy(&x_bytes, &y_bytes)?)),
                Some(other) => Err(CoseCryptoError::InvalidKey(format!(
                    "unsupported EC curve: {other}"
                ))),
                None => Err(CoseCryptoError::InvalidKey("missing crv".to_string())),
            }
        }
        "OKP" => {
            let x = jwk
                .x
                .as_deref()
                .ok_or_else(|| CoseCryptoError::InvalidKey("missing x".to_string()))?;
            let x_bytes = b64url_decode(x)?;

            match jwk.crv.as_deref() {
                Some("Ed25519") => Ok(Box::new(Ed25519Verifier::from_bytes(&x_bytes)?)),
                Some(other) => Err(CoseCryptoError::InvalidKey(format!(
                    "unsupported OKP curve: {other}"
                ))),
                None => Err(CoseCryptoError::InvalidKey("missing crv".to_string())),
            }
        }
        other => Err(CoseCryptoError::InvalidKey(format!(
            "unsupported kty: {other}"
        ))),
    }
}

#[cfg(all(test, feature = "ecdsa"))]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const EC_P256_JWK: &[u8] = br#"{
        "kty": "EC",
        "crv": "P-256",
        "x": "MKBCTNIcKUSDii11ySs3526iDZ8AiTo7Tu6KPAqv7D4",
        "y": "4Etl6SRW2YiLUrN5vfvVHuhp7x8PxltmWWlbbM4IFyM",
        "d": "870MB6gfuTJ4HtUnUvYMyJpr5eUZNP4Bk43bVdj3eAE"
    }"#;

    const EC_P256_PUB_JWK: &[u8] = br#"{
        "kty": "EC",
        "crv": "P-256",
        "x": "MKBCTNIcKUSDii11ySs3526iDZ8AiTo7Tu6KPAqv7D4",
        "y": "4Etl6SRW2YiLUrN5vfvVHuhp7x8PxltmWWlbbM4IFyM"
    }"#;

    #[test]
    fn test_algorithm_from_jwk_ec_p256() {
        let alg = algorithm_from_jwk(EC_P256_JWK).unwrap();
        assert_eq!(alg, CoseAlgorithm::Es256);
    }

    #[test]
    fn test_signer_from_jwk_ec_p256() {
        let signer = signer_from_jwk(EC_P256_JWK).unwrap();
        assert_eq!(signer.algorithm(), CoseAlgorithm::Es256);

        // Sign some data
        let data = b"test data";
        let sig = signer.sign(data).unwrap();
        assert!(!sig.is_empty());

        // Verify with corresponding verifier
        let verifier = verifier_from_jwk(EC_P256_JWK).unwrap();
        verifier.verify(data, &sig).unwrap();
    }

    #[test]
    fn test_verifier_from_jwk_public_only() {
        let verifier = verifier_from_jwk(EC_P256_PUB_JWK).unwrap();
        assert_eq!(verifier.algorithm(), CoseAlgorithm::Es256);
    }

    #[test]
    fn test_signer_from_jwk_no_private_key() {
        let result = signer_from_jwk(EC_P256_PUB_JWK);
        assert!(result.is_err());
    }

    #[test]
    fn test_sign1_roundtrip_with_jwk() {
        use crate::sign::{CoseSign1Builder, verify_sign1};

        let signer = signer_from_jwk(EC_P256_JWK).unwrap();
        let verifier = verifier_from_jwk(EC_P256_JWK).unwrap();

        let payload = b"hello world";
        let hdr = crate::helpers::header_with_algorithm(signer.algorithm());

        let signed = CoseSign1Builder::new()
            .payload(payload)
            .protected(hdr)
            .sign(signer.as_ref())
            .unwrap();

        verify_sign1(&signed, verifier.as_ref(), &[]).unwrap();
    }
}
