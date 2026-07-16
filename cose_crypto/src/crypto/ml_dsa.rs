//! ML-DSA signing and verification (ML-DSA-44, ML-DSA-65, ML-DSA-87).
//!
//! Implements [draft-ietf-cose-dilithium-11](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium-11).

use alloc::{string::ToString, vec::Vec};

use ml_dsa::signature::{Signer, Verifier};

use cose::maps::CoseKeyCbor;

use super::{CoseSigner, CoseVerifier};
use crate::{
    algorithm::CoseAlgorithm,
    error::CoseCryptoError,
    keys::{self, ParsedCoseKey},
};

macro_rules! impl_ml_dsa {
    (
        signer = $signer:ident,
        verifier = $verifier:ident,
        dsa = $dsa:ty,
        algo = $algo:expr,
        alg_label = $alg_label:literal,
        doc_name = $doc_name:literal
    ) => {
        #[doc = concat!($doc_name, " signer using a private key.")]
        pub struct $signer {
            key: ml_dsa::SigningKey<$dsa>,
        }

        impl $signer {
            #[doc = concat!("Create from a 32-byte seed via `SigningKey::from_seed`.")]
            pub fn from_seed(seed: &[u8]) -> Result<Self, CoseCryptoError> {
                let seed_array = ml_dsa::B32::try_from(seed).map_err(|_| {
                    CoseCryptoError::InvalidKey("seed must be 32 bytes".to_string())
                })?;
                let key = ml_dsa::SigningKey::<$dsa>::from_seed(&seed_array);
                Ok(Self { key })
            }

            /// Create from a COSE key structure (AKP kty=7).
            pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
                match keys::parse_cose_key(cose_key)? {
                    ParsedCoseKey::AkpPrivate { alg, priv_key, .. } if alg == $alg_label => {
                        Self::from_seed(&priv_key)
                    }
                    _ => Err(CoseCryptoError::KeyMismatch(
                        concat!("expected AKP ", $doc_name, " private key").to_string(),
                    )),
                }
            }
        }

        impl CoseSigner for $signer {
            fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CoseCryptoError> {
                let sig: ml_dsa::Signature<$dsa> = self
                    .key
                    .try_sign(data)
                    .map_err(|e| CoseCryptoError::InvalidKey(e.to_string()))?;
                Ok(sig.encode().to_vec())
            }

            fn algorithm(&self) -> CoseAlgorithm {
                $algo
            }
        }

        #[doc = concat!($doc_name, " verifier using a public key.")]
        pub struct $verifier {
            key: ml_dsa::VerifyingKey<$dsa>,
        }

        impl $verifier {
            #[doc = concat!("Create from encoded public key bytes.")]
            pub fn from_bytes(pub_key: &[u8]) -> Result<Self, CoseCryptoError> {
                let vk_bytes =
                    ml_dsa::EncodedVerifyingKey::<$dsa>::try_from(pub_key).map_err(|_| {
                        CoseCryptoError::InvalidKey(
                            concat!("invalid ", $doc_name, " verifying key length").to_string(),
                        )
                    })?;
                let key = ml_dsa::VerifyingKey::<$dsa>::decode(&vk_bytes);
                Ok(Self { key })
            }

            /// Create from a COSE key structure (AKP kty=7).
            pub fn from_cose_key(cose_key: &CoseKeyCbor) -> Result<Self, CoseCryptoError> {
                match keys::parse_cose_key(cose_key)? {
                    ParsedCoseKey::AkpPublic { alg, pub_key } if alg == $alg_label => {
                        Self::from_bytes(&pub_key)
                    }
                    ParsedCoseKey::AkpPrivate { alg, pub_key, .. } if alg == $alg_label => {
                        Self::from_bytes(&pub_key)
                    }
                    _ => Err(CoseCryptoError::KeyMismatch(
                        concat!("expected AKP ", $doc_name, " key").to_string(),
                    )),
                }
            }
        }

        impl CoseVerifier for $verifier {
            fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), CoseCryptoError> {
                let sig_bytes = ml_dsa::EncodedSignature::<$dsa>::try_from(signature)
                    .map_err(|_| CoseCryptoError::VerificationFailed)?;
                let sig = ml_dsa::Signature::<$dsa>::decode(&sig_bytes)
                    .ok_or(CoseCryptoError::VerificationFailed)?;
                self.key
                    .verify(data, &sig)
                    .map_err(|_| CoseCryptoError::VerificationFailed)
            }

            fn algorithm(&self) -> CoseAlgorithm {
                $algo
            }
        }
    };
}

impl_ml_dsa!(
    signer = MlDsa44Signer,
    verifier = MlDsa44Verifier,
    dsa = ml_dsa::MlDsa44,
    algo = CoseAlgorithm::MlDsa44,
    alg_label = -48,
    doc_name = "ML-DSA-44"
);

impl_ml_dsa!(
    signer = MlDsa65Signer,
    verifier = MlDsa65Verifier,
    dsa = ml_dsa::MlDsa65,
    algo = CoseAlgorithm::MlDsa65,
    alg_label = -49,
    doc_name = "ML-DSA-65"
);

impl_ml_dsa!(
    signer = MlDsa87Signer,
    verifier = MlDsa87Verifier,
    dsa = ml_dsa::MlDsa87,
    algo = CoseAlgorithm::MlDsa87,
    alg_label = -50,
    doc_name = "ML-DSA-87"
);
