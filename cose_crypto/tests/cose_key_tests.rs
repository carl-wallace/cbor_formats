//! Tests for COSE Key support: signer_from_cose_key, verifier_from_cose_key,
//! algorithm_from_cose_key. Includes cose-wg test vector verification and
//! sign/verify roundtrips using COSE Key CBOR files.

#![allow(clippy::unwrap_used)]

use ciborium::de::from_reader;

use cose::maps::CoseKeyCbor;
use cose_crypto::{
    algorithm::CoseAlgorithm,
    helpers,
    keys::{algorithm_from_cose_key, signer_from_cose_key, verifier_from_cose_key},
    sign::{CoseSign1Builder, verify_sign1},
};

/// Parse a COSE Key from CBOR bytes.
fn parse_cose_key(cbor: &[u8]) -> CoseKeyCbor {
    from_reader(cbor).expect("failed to parse COSE Key CBOR")
}

// ── cose-wg test vector: P-256 sign/verify roundtrip ──

#[test]
fn cose_wg_p256_sign_verify_roundtrip() {
    let key_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-p256.cosekey");
    let pub_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-p256-pub.cosekey");

    let priv_key = parse_cose_key(key_cbor);
    let pub_key = parse_cose_key(pub_cbor);

    // Algorithm detection
    assert_eq!(
        algorithm_from_cose_key(&priv_key).unwrap(),
        CoseAlgorithm::Es256
    );
    assert_eq!(
        algorithm_from_cose_key(&pub_key).unwrap(),
        CoseAlgorithm::Es256
    );

    // Sign with private key
    let signer = signer_from_cose_key(&priv_key).unwrap();
    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(signer.as_ref())
        .unwrap();

    // Verify with private key (extracts public components)
    let verifier_from_priv = verifier_from_cose_key(&priv_key).unwrap();
    verify_sign1(&sign1, verifier_from_priv.as_ref(), &[]).unwrap();

    // Verify with public-only key
    let verifier_from_pub = verifier_from_cose_key(&pub_key).unwrap();
    verify_sign1(&sign1, verifier_from_pub.as_ref(), &[]).unwrap();

    // Public-only key cannot sign
    assert!(signer_from_cose_key(&pub_key).is_err());
}

// ── cose-wg test vector: Ed25519 sign/verify roundtrip ──

#[test]
fn cose_wg_ed25519_sign_verify_roundtrip() {
    let key_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-ed25519.cosekey");
    let pub_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-ed25519-pub.cosekey");

    let priv_key = parse_cose_key(key_cbor);
    let pub_key = parse_cose_key(pub_cbor);

    // Algorithm detection
    assert_eq!(
        algorithm_from_cose_key(&priv_key).unwrap(),
        CoseAlgorithm::Eddsa
    );
    assert_eq!(
        algorithm_from_cose_key(&pub_key).unwrap(),
        CoseAlgorithm::Eddsa
    );

    // Sign with private key
    let signer = signer_from_cose_key(&priv_key).unwrap();
    let payload = b"This is the content.";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Eddsa);

    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(signer.as_ref())
        .unwrap();

    // Verify with private key
    let verifier = verifier_from_cose_key(&priv_key).unwrap();
    verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();

    // Verify with public-only key
    let pub_verifier = verifier_from_cose_key(&pub_key).unwrap();
    verify_sign1(&sign1, pub_verifier.as_ref(), &[]).unwrap();

    // Public-only key cannot sign
    assert!(signer_from_cose_key(&pub_key).is_err());
}

// ── Verify cose-wg COSE_Sign1 test vector (ES256) ──
// From cose-wg/Examples ecdsa-sig-01: COSE_Sign1 with P-256 key, payload "This is the content."

#[test]
fn cose_wg_verify_es256_sign1_vector() {
    // COSE_Sign1 from ecdsa-sig-01.json (cose-wg/Examples)
    let sign1_hex = "D28445A201260300A10442313154546869732069732074686520636F6E74656E742E58406520BBAF2081D7E0ED0F95F76EB0733D667005F7467CEC4B87B9381A6BA1EDE8E00DF29F32A37230F39A842A54821FDD223092819D7728EFB9D3A0080B75380B";
    let sign1_bytes = hex::decode(sign1_hex).expect("hex decode");

    let sign1: cose::arrays::CoseSign1Cbor =
        from_reader(sign1_bytes.as_slice()).expect("CBOR decode");

    let pub_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-p256-pub.cosekey");
    let pub_key = parse_cose_key(pub_cbor);
    let verifier = verifier_from_cose_key(&pub_key).unwrap();

    verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
}

// ── Verify cose-wg COSE_Sign1 test vector (EdDSA) ──
// From cose-wg/Examples eddsa-sig-01: COSE_Sign1 with Ed25519 key

#[test]
fn cose_wg_verify_eddsa_sign1_vector() {
    let sign1_hex = "D28445A201270300A10442313154546869732069732074686520636F6E74656E742E58407142FD2FF96D56DB85BEE905A76BA1D0B7321A95C8C4D3607C5781932B7AFB8711497DFA751BF40B58B3BCC32300B1487F3DB34085EEF013BF08F4A44D6FEF0D";
    let sign1_bytes = hex::decode(sign1_hex).expect("hex decode");

    let sign1: cose::arrays::CoseSign1Cbor =
        from_reader(sign1_bytes.as_slice()).expect("CBOR decode");

    let pub_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-ed25519-pub.cosekey");
    let pub_key = parse_cose_key(pub_cbor);
    let verifier = verifier_from_cose_key(&pub_key).unwrap();

    verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
}

// ── Cross-format: sign with JWK-converted COSE Key, verify with fresh COSE Key ──

#[test]
fn sign_with_jwk_converted_cose_key_es256() {
    let key_cbor = include_bytes!("../../cfcli/tests/data/keys/es256-from-jwk.cosekey");
    let key = parse_cose_key(key_cbor);

    assert_eq!(algorithm_from_cose_key(&key).unwrap(), CoseAlgorithm::Es256);

    let signer = signer_from_cose_key(&key).unwrap();
    let verifier = verifier_from_cose_key(&key).unwrap();

    let payload = b"cross-format test payload";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(signer.as_ref())
        .unwrap();

    verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
}

#[test]
fn sign_with_jwk_converted_cose_key_es384() {
    let key_cbor = include_bytes!("../../cfcli/tests/data/keys/es384-from-jwk.cosekey");
    let key = parse_cose_key(key_cbor);

    assert_eq!(algorithm_from_cose_key(&key).unwrap(), CoseAlgorithm::Es384);

    let signer = signer_from_cose_key(&key).unwrap();
    let verifier = verifier_from_cose_key(&key).unwrap();

    let payload = b"P-384 cross-format test";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es384);

    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(signer.as_ref())
        .unwrap();

    verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
}

#[test]
fn sign_with_jwk_converted_cose_key_ed25519() {
    let key_cbor = include_bytes!("../../cfcli/tests/data/keys/ed25519-from-jwk.cosekey");
    let key = parse_cose_key(key_cbor);

    assert_eq!(algorithm_from_cose_key(&key).unwrap(), CoseAlgorithm::Eddsa);

    let signer = signer_from_cose_key(&key).unwrap();
    let verifier = verifier_from_cose_key(&key).unwrap();

    let payload = b"Ed25519 cross-format test";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Eddsa);

    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(signer.as_ref())
        .unwrap();

    verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
}

// ── Cross-format: sign with JWK, verify with COSE Key (same underlying key) ──

#[test]
fn cross_format_jwk_sign_cose_key_verify() {
    use cose_crypto::jwk::signer_from_jwk;

    // Sign with the original JWK
    let jwk_bytes = include_bytes!("../../cfcli/tests/data/keys/es256.jwk");
    let jwk_signer = signer_from_jwk(jwk_bytes).unwrap();

    let payload = b"cross-format interop";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(jwk_signer.as_ref())
        .unwrap();

    // Verify with the COSE Key converted from that same JWK
    let cose_key_bytes = include_bytes!("../../cfcli/tests/data/keys/es256-from-jwk.cosekey");
    let cose_key = parse_cose_key(cose_key_bytes);
    let cose_verifier = verifier_from_cose_key(&cose_key).unwrap();

    verify_sign1(&sign1, cose_verifier.as_ref(), &[]).unwrap();
}

// ── Wrong key fails ──

#[test]
fn cose_key_wrong_key_verification_fails() {
    // Sign with cose-wg P-256 key
    let key_cbor = include_bytes!("../../cfcli/tests/data/keys/cose-wg-p256.cosekey");
    let key = parse_cose_key(key_cbor);
    let signer = signer_from_cose_key(&key).unwrap();

    let payload = b"signed data";
    let protected = helpers::header_with_algorithm(CoseAlgorithm::Es256);
    let sign1 = CoseSign1Builder::new()
        .payload(payload)
        .protected(protected)
        .sign(signer.as_ref())
        .unwrap();

    // Try to verify with a different key (freshly generated es256)
    let wrong_key_cbor = include_bytes!("../../cfcli/tests/data/keys/es256.cosekey");
    let wrong_key = parse_cose_key(wrong_key_cbor);
    let wrong_verifier = verifier_from_cose_key(&wrong_key).unwrap();

    assert!(verify_sign1(&sign1, wrong_verifier.as_ref(), &[]).is_err());
}

// ══════════════════════════════════════════════════════════════
// ML-DSA (Post-Quantum) tests — gated behind the `pqc` feature
// ══════════════════════════════════════════════════════════════

#[cfg(feature = "pqc")]
mod ml_dsa {
    use super::*;

    /// Load a COSE Key from the draft-ietf-cose-dilithium test vector JSON.
    fn load_spec_cose_key(variant: &str) -> CoseKeyCbor {
        let path = format!("./tests/examples/{variant}.cose.json");
        let data =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
        let raw: serde_json::Value = serde_json::from_str(&data).unwrap();
        let key_hex = raw["key"].as_str().unwrap();
        let key_bytes = hex::decode(key_hex).unwrap();
        from_reader(key_bytes.as_slice()).unwrap()
    }

    /// Load a COSE_Sign1 from the draft-ietf-cose-dilithium test vector JSON.
    fn load_spec_sign1(variant: &str) -> cose::arrays::CoseSign1Cbor {
        let path = format!("./tests/examples/{variant}.cose.json");
        let data =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
        let raw: serde_json::Value = serde_json::from_str(&data).unwrap();
        let sign1_hex = raw["sign1"].as_str().unwrap();
        let sign1_bytes = hex::decode(sign1_hex).unwrap();
        // Handle optional CBOR tag 18 wrapper
        match from_reader::<cose::arrays::CoseSign1Cbor, _>(sign1_bytes.as_slice()) {
            Ok(s) => s,
            Err(_) => {
                let value: ciborium::value::Value = from_reader(sign1_bytes.as_slice()).unwrap();
                if let ciborium::value::Value::Tag(18, inner) = value {
                    let mut buf = vec![];
                    ciborium::ser::into_writer(&*inner, &mut buf).unwrap();
                    from_reader(buf.as_slice()).unwrap()
                } else {
                    panic!("expected COSE_Sign1 or tag 18 wrapper");
                }
            }
        }
    }

    // ── ML-DSA-44 ──

    #[test]
    fn ml_dsa_44_algorithm_from_cose_key() {
        let key = load_spec_cose_key("ML_DSA_44");
        assert_eq!(
            algorithm_from_cose_key(&key).unwrap(),
            CoseAlgorithm::MlDsa44
        );
    }

    #[test]
    fn ml_dsa_44_sign_verify_roundtrip() {
        let key = load_spec_cose_key("ML_DSA_44");

        let signer = signer_from_cose_key(&key).unwrap();
        assert_eq!(signer.algorithm(), CoseAlgorithm::MlDsa44);

        let verifier = verifier_from_cose_key(&key).unwrap();
        assert_eq!(verifier.algorithm(), CoseAlgorithm::MlDsa44);

        let payload = b"ML-DSA-44 test payload";
        let protected = helpers::header_with_algorithm(CoseAlgorithm::MlDsa44);

        let sign1 = CoseSign1Builder::new()
            .payload(payload)
            .protected(protected)
            .sign(signer.as_ref())
            .unwrap();

        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }

    #[test]
    fn ml_dsa_44_verify_spec_sign1_via_keys_api() {
        let key = load_spec_cose_key("ML_DSA_44");
        let sign1 = load_spec_sign1("ML_DSA_44");

        let verifier = verifier_from_cose_key(&key).unwrap();
        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }

    #[test]
    fn ml_dsa_44_wrong_key_fails() {
        let key44 = load_spec_cose_key("ML_DSA_44");
        let signer = signer_from_cose_key(&key44).unwrap();

        let payload = b"signed with ML-DSA-44";
        let protected = helpers::header_with_algorithm(CoseAlgorithm::MlDsa44);
        let sign1 = CoseSign1Builder::new()
            .payload(payload)
            .protected(protected)
            .sign(signer.as_ref())
            .unwrap();

        // ML-DSA-65 key should not verify an ML-DSA-44 signature
        let key65 = load_spec_cose_key("ML_DSA_65");
        let wrong_verifier = verifier_from_cose_key(&key65).unwrap();
        assert!(verify_sign1(&sign1, wrong_verifier.as_ref(), &[]).is_err());
    }

    // ── ML-DSA-65 ──

    #[test]
    fn ml_dsa_65_algorithm_from_cose_key() {
        let key = load_spec_cose_key("ML_DSA_65");
        assert_eq!(
            algorithm_from_cose_key(&key).unwrap(),
            CoseAlgorithm::MlDsa65
        );
    }

    #[test]
    fn ml_dsa_65_sign_verify_roundtrip() {
        let key = load_spec_cose_key("ML_DSA_65");

        let signer = signer_from_cose_key(&key).unwrap();
        let verifier = verifier_from_cose_key(&key).unwrap();

        let payload = b"ML-DSA-65 test payload";
        let protected = helpers::header_with_algorithm(CoseAlgorithm::MlDsa65);

        let sign1 = CoseSign1Builder::new()
            .payload(payload)
            .protected(protected)
            .sign(signer.as_ref())
            .unwrap();

        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }

    #[test]
    fn ml_dsa_65_verify_spec_sign1_via_keys_api() {
        let key = load_spec_cose_key("ML_DSA_65");
        let sign1 = load_spec_sign1("ML_DSA_65");

        let verifier = verifier_from_cose_key(&key).unwrap();
        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }

    // ── ML-DSA-87 ──

    #[test]
    fn ml_dsa_87_algorithm_from_cose_key() {
        let key = load_spec_cose_key("ML_DSA_87");
        assert_eq!(
            algorithm_from_cose_key(&key).unwrap(),
            CoseAlgorithm::MlDsa87
        );
    }

    #[test]
    fn ml_dsa_87_sign_verify_roundtrip() {
        let key = load_spec_cose_key("ML_DSA_87");

        let signer = signer_from_cose_key(&key).unwrap();
        let verifier = verifier_from_cose_key(&key).unwrap();

        let payload = b"ML-DSA-87 test payload";
        let protected = helpers::header_with_algorithm(CoseAlgorithm::MlDsa87);

        let sign1 = CoseSign1Builder::new()
            .payload(payload)
            .protected(protected)
            .sign(signer.as_ref())
            .unwrap();

        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }

    #[test]
    fn ml_dsa_87_verify_spec_sign1_via_keys_api() {
        let key = load_spec_cose_key("ML_DSA_87");
        let sign1 = load_spec_sign1("ML_DSA_87");

        let verifier = verifier_from_cose_key(&key).unwrap();
        verify_sign1(&sign1, verifier.as_ref(), &[]).unwrap();
    }
}
