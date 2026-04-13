//! COSE key parsing — extracts key material from `CoseKeyCbor` structures.

use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use ciborium::value::Value;
use cose::maps::CoseKeyCbor;
use zeroize::{Zeroize, Zeroizing};

use crate::algorithm::CoseAlgorithm;
use crate::crypto::{CoseSigner, CoseVerifier};
use crate::error::CoseCryptoError;

/// COSE key type values (kty parameter, label 1).
const KTY_OKP: i64 = 1;
const KTY_EC2: i64 = 2;
const KTY_SYMMETRIC: i64 = 4;
/// AKP key type (kty=7) for algorithm key pairs (ML-DSA, etc.)
#[cfg(feature = "pqc")]
const KTY_AKP: i64 = 7;

/// EC2/OKP curve values (crv parameter, label -1).
const CRV_P256: i64 = 1;
const CRV_P384: i64 = 2;
const CRV_ED25519: i64 = 6;

/// Parsed COSE key material.
#[derive(Debug)]
pub enum ParsedCoseKey {
    /// EC private key: (curve, x, y, d)
    EcPrivate {
        /// Curve identifier
        crv: i64,
        /// X coordinate
        x: Vec<u8>,
        /// Y coordinate
        y: Vec<u8>,
        /// Private key scalar
        d: Vec<u8>,
    },
    /// EC public key: (curve, x, y)
    EcPublic {
        /// Curve identifier
        crv: i64,
        /// X coordinate
        x: Vec<u8>,
        /// Y coordinate
        y: Vec<u8>,
    },
    /// OKP private key: (curve, x, d)
    OkpPrivate {
        /// Curve identifier
        crv: i64,
        /// Public key bytes
        x: Vec<u8>,
        /// Private key bytes
        d: Vec<u8>,
    },
    /// OKP public key: (curve, x)
    OkpPublic {
        /// Curve identifier
        crv: i64,
        /// Public key bytes
        x: Vec<u8>,
    },
    /// Symmetric key: (k)
    Symmetric {
        /// Key bytes
        k: Vec<u8>,
    },
    /// AKP private key: (alg, pub, priv)
    #[cfg(feature = "pqc")]
    AkpPrivate {
        /// Algorithm identifier (required for AKP keys)
        alg: i64,
        /// Public key bytes (label -1)
        pub_key: Vec<u8>,
        /// Private key bytes (label -2)
        priv_key: Vec<u8>,
    },
    /// AKP public key: (alg, pub)
    #[cfg(feature = "pqc")]
    AkpPublic {
        /// Algorithm identifier (required for AKP keys)
        alg: i64,
        /// Public key bytes (label -1)
        pub_key: Vec<u8>,
    },
}

impl Zeroize for ParsedCoseKey {
    fn zeroize(&mut self) {
        match self {
            Self::EcPrivate { d, x, y, .. } => {
                d.zeroize();
                x.zeroize();
                y.zeroize();
            }
            Self::EcPublic { x, y, .. } => {
                x.zeroize();
                y.zeroize();
            }
            Self::OkpPrivate { d, x, .. } => {
                d.zeroize();
                x.zeroize();
            }
            Self::OkpPublic { x, .. } => x.zeroize(),
            Self::Symmetric { k } => k.zeroize(),
            #[cfg(feature = "pqc")]
            Self::AkpPrivate {
                pub_key, priv_key, ..
            } => {
                pub_key.zeroize();
                priv_key.zeroize();
            }
            #[cfg(feature = "pqc")]
            Self::AkpPublic { pub_key, .. } => pub_key.zeroize(),
        }
    }
}

/// Extract a parameter value from the `other` field of a CoseKeyCbor by integer label.
fn get_param(key: &CoseKeyCbor, label: i64) -> Option<Value> {
    key.other.as_ref()?.iter().find_map(|t| {
        let key_int: i64 = match &t.key {
            Value::Integer(i) => (*i).try_into().ok()?,
            _ => return None,
        };
        if key_int == label {
            Some(t.value.clone())
        } else {
            None
        }
    })
}

/// Extract bytes from a CBOR Value.
fn value_to_bytes(v: &Value) -> Result<Vec<u8>, CoseCryptoError> {
    v.as_bytes()
        .cloned()
        .ok_or_else(|| CoseCryptoError::InvalidKey("expected bytes parameter".to_string()))
}

/// Extract an integer from a CBOR Value.
fn value_to_i64(v: &Value) -> Result<i64, CoseCryptoError> {
    v.as_integer()
        .and_then(|i| i64::try_from(i).ok())
        .ok_or_else(|| CoseCryptoError::InvalidKey("expected integer parameter".to_string()))
}

/// Extract the key type (kty) from a CoseKeyCbor.
fn get_kty(key: &CoseKeyCbor) -> Result<i64, CoseCryptoError> {
    match &key.kty {
        Some(common::TextOrInt::Int(i)) => Ok(*i),
        Some(common::TextOrInt::Text(_)) => Err(CoseCryptoError::InvalidKey(
            "text kty not supported".to_string(),
        )),
        None => Err(CoseCryptoError::InvalidKey("missing kty".to_string())),
    }
}

/// Parse a `CoseKeyCbor` into structured key material.
pub fn parse_cose_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let kty = get_kty(key)?;
    match kty {
        KTY_EC2 => parse_ec2_key(key),
        KTY_OKP => parse_okp_key(key),
        KTY_SYMMETRIC => parse_symmetric_key(key),
        #[cfg(feature = "pqc")]
        KTY_AKP => parse_akp_key(key),
        _ => Err(CoseCryptoError::InvalidKey(alloc::format!(
            "unsupported kty: {kty}"
        ))),
    }
}

fn parse_ec2_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let crv_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing crv (-1)".to_string()))?;
    let crv = value_to_i64(&crv_val)?;

    let x_val = get_param(key, -2)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing x (-2)".to_string()))?;
    let x = value_to_bytes(&x_val)?;

    let y_val = get_param(key, -3)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing y (-3)".to_string()))?;
    let y = value_to_bytes(&y_val)?;

    match get_param(key, -4) {
        Some(d_val) => {
            let d = value_to_bytes(&d_val)?;
            Ok(ParsedCoseKey::EcPrivate { crv, x, y, d })
        }
        None => Ok(ParsedCoseKey::EcPublic { crv, x, y }),
    }
}

fn parse_okp_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let crv_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing crv (-1)".to_string()))?;
    let crv = value_to_i64(&crv_val)?;

    let x_val = get_param(key, -2)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing x (-2)".to_string()))?;
    let x = value_to_bytes(&x_val)?;

    match get_param(key, -4) {
        Some(d_val) => {
            let d = value_to_bytes(&d_val)?;
            Ok(ParsedCoseKey::OkpPrivate { crv, x, d })
        }
        None => Ok(ParsedCoseKey::OkpPublic { crv, x }),
    }
}

fn parse_symmetric_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    let k_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing k (-1)".to_string()))?;
    let k = value_to_bytes(&k_val)?;
    Ok(ParsedCoseKey::Symmetric { k })
}

#[cfg(feature = "pqc")]
fn parse_akp_key(key: &CoseKeyCbor) -> Result<ParsedCoseKey, CoseCryptoError> {
    // AKP keys require the alg parameter (label 3) to distinguish variants
    let alg = match &key.alg {
        Some(common::TextOrInt::Int(i)) => *i,
        _ => {
            return Err(CoseCryptoError::InvalidKey(
                "AKP keys require alg parameter".to_string(),
            ));
        }
    };

    let pub_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing pub (-1)".to_string()))?;
    let pub_key = value_to_bytes(&pub_val)?;

    match get_param(key, -2) {
        Some(priv_val) => {
            let priv_key = value_to_bytes(&priv_val)?;
            Ok(ParsedCoseKey::AkpPrivate {
                alg,
                pub_key,
                priv_key,
            })
        }
        None => Ok(ParsedCoseKey::AkpPublic { alg, pub_key }),
    }
}

/// Determine the COSE algorithm from a `CoseKeyCbor`.
///
/// For EC2 and OKP keys the algorithm is inferred from the curve.
/// For AKP keys (PQC) the `alg` field is required.
/// Symmetric keys require an explicit `alg` field since the key size alone
/// is ambiguous (e.g. 32 bytes could be HMAC-SHA-256 or AES-256-GCM).
pub fn algorithm_from_cose_key(key: &CoseKeyCbor) -> Result<CoseAlgorithm, CoseCryptoError> {
    let parsed = Zeroizing::new(parse_cose_key(key)?);
    match &*parsed {
        ParsedCoseKey::EcPrivate { crv, .. } | ParsedCoseKey::EcPublic { crv, .. } => match *crv {
            CRV_P256 => Ok(CoseAlgorithm::Es256),
            CRV_P384 => Ok(CoseAlgorithm::Es384),
            other => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported EC2 curve: {other}"
            ))),
        },
        ParsedCoseKey::OkpPrivate { crv, .. } | ParsedCoseKey::OkpPublic { crv, .. } => {
            match *crv {
                CRV_ED25519 => Ok(CoseAlgorithm::Eddsa),
                other => Err(CoseCryptoError::InvalidKey(format!(
                    "unsupported OKP curve: {other}"
                ))),
            }
        }
        ParsedCoseKey::Symmetric { .. } => {
            // Symmetric keys need an explicit alg to disambiguate.
            match &key.alg {
                Some(toi) => CoseAlgorithm::from_text_or_int(toi),
                None => Err(CoseCryptoError::InvalidKey(
                    "symmetric COSE Key requires alg field".to_string(),
                )),
            }
        }
        #[cfg(feature = "pqc")]
        ParsedCoseKey::AkpPrivate { alg, .. } | ParsedCoseKey::AkpPublic { alg, .. } => {
            CoseAlgorithm::from_i64(*alg)
        }
    }
}

/// Create a [`CoseSigner`] from a `CoseKeyCbor`.
///
/// The key must contain private key material.
pub fn signer_from_cose_key(key: &CoseKeyCbor) -> Result<Box<dyn CoseSigner>, CoseCryptoError> {
    use crate::crypto::ecdsa::{Es256Signer, Es384Signer};
    use crate::crypto::eddsa::Ed25519Signer;

    let parsed = Zeroizing::new(parse_cose_key(key)?);
    match &*parsed {
        ParsedCoseKey::EcPrivate { crv, d, .. } => match crv {
            &CRV_P256 => Ok(Box::new(Es256Signer::from_bytes(d)?)),
            &CRV_P384 => Ok(Box::new(Es384Signer::from_bytes(d)?)),
            other => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported EC2 curve: {other}"
            ))),
        },
        ParsedCoseKey::OkpPrivate { crv, d, .. } => match crv {
            &CRV_ED25519 => Ok(Box::new(Ed25519Signer::from_bytes(d)?)),
            other => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported OKP curve: {other}"
            ))),
        },
        #[cfg(feature = "pqc")]
        ParsedCoseKey::AkpPrivate { alg, priv_key, .. } => {
            use crate::crypto::ml_dsa::{MlDsa44Signer, MlDsa65Signer, MlDsa87Signer};
            match CoseAlgorithm::from_i64(*alg)? {
                CoseAlgorithm::MlDsa44 => Ok(Box::new(MlDsa44Signer::from_seed(priv_key)?)),
                CoseAlgorithm::MlDsa65 => Ok(Box::new(MlDsa65Signer::from_seed(priv_key)?)),
                CoseAlgorithm::MlDsa87 => Ok(Box::new(MlDsa87Signer::from_seed(priv_key)?)),
                _ => Err(CoseCryptoError::InvalidKey(format!(
                    "unsupported AKP algorithm: {alg}"
                ))),
            }
        }
        _ => Err(CoseCryptoError::InvalidKey(
            "COSE Key does not contain private key material".to_string(),
        )),
    }
}

/// Create a [`CoseVerifier`] from a `CoseKeyCbor`.
///
/// Uses the public key components. If a private key is present, the public
/// components are still used for verification.
pub fn verifier_from_cose_key(key: &CoseKeyCbor) -> Result<Box<dyn CoseVerifier>, CoseCryptoError> {
    use crate::crypto::ecdsa::{Es256Verifier, Es384Verifier};
    use crate::crypto::eddsa::Ed25519Verifier;

    let parsed = Zeroizing::new(parse_cose_key(key)?);
    match &*parsed {
        ParsedCoseKey::EcPublic { crv, x, y } | ParsedCoseKey::EcPrivate { crv, x, y, .. } => {
            match crv {
                &CRV_P256 => Ok(Box::new(Es256Verifier::from_xy(x, y)?)),
                &CRV_P384 => Ok(Box::new(Es384Verifier::from_xy(x, y)?)),
                other => Err(CoseCryptoError::InvalidKey(format!(
                    "unsupported EC2 curve: {other}"
                ))),
            }
        }
        ParsedCoseKey::OkpPublic { crv, x } | ParsedCoseKey::OkpPrivate { crv, x, .. } => match crv
        {
            &CRV_ED25519 => Ok(Box::new(Ed25519Verifier::from_bytes(x)?)),
            other => Err(CoseCryptoError::InvalidKey(format!(
                "unsupported OKP curve: {other}"
            ))),
        },
        #[cfg(feature = "pqc")]
        ParsedCoseKey::AkpPublic { alg, pub_key }
        | ParsedCoseKey::AkpPrivate { alg, pub_key, .. } => {
            use crate::crypto::ml_dsa::{MlDsa44Verifier, MlDsa65Verifier, MlDsa87Verifier};
            match CoseAlgorithm::from_i64(*alg)? {
                CoseAlgorithm::MlDsa44 => Ok(Box::new(MlDsa44Verifier::from_bytes(pub_key)?)),
                CoseAlgorithm::MlDsa65 => Ok(Box::new(MlDsa65Verifier::from_bytes(pub_key)?)),
                CoseAlgorithm::MlDsa87 => Ok(Box::new(MlDsa87Verifier::from_bytes(pub_key)?)),
                _ => Err(CoseCryptoError::InvalidKey(format!(
                    "unsupported AKP algorithm: {alg}"
                ))),
            }
        }
        ParsedCoseKey::Symmetric { .. } => Err(CoseCryptoError::InvalidKey(
            "symmetric keys cannot be used for signing/verification".to_string(),
        )),
    }
}

/// Returns the curve identifier for the given CoseKeyCbor, if it's an EC2 or OKP key.
pub fn get_crv(key: &CoseKeyCbor) -> Result<i64, CoseCryptoError> {
    let crv_val = get_param(key, -1)
        .ok_or_else(|| CoseCryptoError::InvalidKey("missing crv (-1)".to_string()))?;
    value_to_i64(&crv_val)
}

/// Re-export curve constants for use by other modules.
pub use self::constants::*;
mod constants {
    /// COSE curve identifier for P-256
    pub const P256: i64 = super::CRV_P256;
    /// COSE curve identifier for P-384
    pub const P384: i64 = super::CRV_P384;
    /// COSE curve identifier for Ed25519
    pub const ED25519: i64 = super::CRV_ED25519;
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::sign::{CoseSign1Builder, verify_sign1};
    use common::{TextOrInt, TupleCbor};

    /// Build a CoseKeyCbor with the given kty and key-specific parameters.
    fn build_cose_key(kty: i64, alg: Option<i64>, params: Vec<(i64, Value)>) -> CoseKeyCbor {
        let other: Vec<TupleCbor> = params
            .into_iter()
            .map(|(label, value)| TupleCbor {
                key: Value::Integer(label.into()),
                value,
            })
            .collect();
        CoseKeyCbor {
            kty: Some(TextOrInt::Int(kty)),
            kid: None,
            alg: alg.map(TextOrInt::Int),
            key_ops: None,
            iv: None,
            other: Some(other),
        }
    }

    #[test]
    fn cose_key_es256_sign_verify_roundtrip() {
        // Generate a P-256 key
        use elliptic_curve::Generate;
        use elliptic_curve::sec1::Coordinates;
        let sk = p256::ecdsa::SigningKey::generate();
        let vk = sk.verifying_key();
        let pt = vk.to_sec1_point(false);
        let (x, y) = match pt.coordinates() {
            Coordinates::Uncompressed { x, y } => (x.to_vec(), y.to_vec()),
            _ => panic!("unexpected"),
        };
        let d = sk.to_bytes().to_vec();

        let cose_key = build_cose_key(
            2,
            None,
            vec![
                (-1, Value::Integer(CRV_P256.into())),
                (-2, Value::Bytes(x.clone())),
                (-3, Value::Bytes(y.clone())),
                (-4, Value::Bytes(d)),
            ],
        );

        assert_eq!(
            algorithm_from_cose_key(&cose_key).unwrap(),
            CoseAlgorithm::Es256
        );

        let signer = signer_from_cose_key(&cose_key).unwrap();
        let verifier = verifier_from_cose_key(&cose_key).unwrap();

        let payload = b"test payload";
        let hdr = crate::helpers::header_with_algorithm(signer.algorithm());
        let sign1 = CoseSign1Builder::new()
            .payload(payload)
            .protected(hdr)
            .sign(signer.as_ref())
            .unwrap();
        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();

        // Also test public-only key for verification
        let pub_key = build_cose_key(
            2,
            None,
            vec![
                (-1, Value::Integer(CRV_P256.into())),
                (-2, Value::Bytes(x)),
                (-3, Value::Bytes(y)),
            ],
        );
        let pub_verifier = verifier_from_cose_key(&pub_key).unwrap();
        verify_sign1(&sign1, pub_verifier.as_ref(), &[]).unwrap();

        // Public-only key should fail as signer
        assert!(signer_from_cose_key(&pub_key).is_err());
    }

    #[test]
    fn cose_key_ed25519_sign_verify_roundtrip() {
        let mut rng = rand::rng();
        let sk = ed25519_dalek::SigningKey::generate(&mut rng);
        let vk = sk.verifying_key();

        let cose_key = build_cose_key(
            1,
            None,
            vec![
                (-1, Value::Integer(CRV_ED25519.into())),
                (-2, Value::Bytes(vk.as_bytes().to_vec())),
                (-4, Value::Bytes(sk.to_bytes().to_vec())),
            ],
        );

        assert_eq!(
            algorithm_from_cose_key(&cose_key).unwrap(),
            CoseAlgorithm::Eddsa
        );

        let signer = signer_from_cose_key(&cose_key).unwrap();
        let verifier = verifier_from_cose_key(&cose_key).unwrap();

        let payload = b"ed25519 test";
        let hdr = crate::helpers::header_with_algorithm(signer.algorithm());
        let sign1 = CoseSign1Builder::new()
            .payload(payload)
            .protected(hdr)
            .sign(signer.as_ref())
            .unwrap();
        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }

    #[test]
    fn cose_key_symmetric_requires_alg() {
        let cose_key = build_cose_key(4, None, vec![(-1, Value::Bytes(vec![0u8; 32]))]);
        // Should fail: no alg field
        assert!(algorithm_from_cose_key(&cose_key).is_err());

        // With alg field
        let cose_key_with_alg = build_cose_key(4, Some(5), vec![(-1, Value::Bytes(vec![0u8; 32]))]);
        assert_eq!(
            algorithm_from_cose_key(&cose_key_with_alg).unwrap(),
            CoseAlgorithm::Hs256
        );

        // Symmetric keys should fail as signer/verifier
        assert!(signer_from_cose_key(&cose_key_with_alg).is_err());
        assert!(verifier_from_cose_key(&cose_key_with_alg).is_err());
    }
}
