//! Tests for ML-DSA signing and verification using spec test vectors from
//! draft-ietf-cose-dilithium-11 (https://github.com/cose-wg/draft-ietf-cose-dilithium).

#![cfg(feature = "pqc")]
#![allow(clippy::unwrap_used)]

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use cose::arrays::CoseSign1Cbor;
use cose::maps::CoseKeyCbor;
use cose_crypto::algorithm::CoseAlgorithm;
use cose_crypto::crypto::ml_dsa::*;
use cose_crypto::sign::verify_sign1;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoseTestVector {
    /// 32-byte seed (hex)
    priv_key: String,
    /// COSE_Key (hex)
    key: String,
    /// COSE_Sign1 (hex)
    sign1: String,
    /// Raw public key bytes (hex)
    raw_public_key: String,
    /// Raw signature bytes (hex)
    raw_signature: String,
    /// Raw Sig_structure bytes (hex)
    raw_to_be_signed: String,
}

/// Direct verification of raw signature against raw to-be-signed bytes,
/// bypassing our Sig_structure construction.
fn verify_raw(verifier: &dyn cose_crypto::crypto::CoseVerifier, tbs: &[u8], sig: &[u8]) {
    verifier.verify(tbs, sig).unwrap();
}

fn load_vector(name: &str) -> CoseTestVector {
    let path = format!("./tests/examples/{name}.cose.json");
    let data =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    let raw: serde_json::Value = serde_json::from_str(&data).unwrap();
    CoseTestVector {
        priv_key: raw["priv"].as_str().unwrap().to_string(),
        key: raw["key"].as_str().unwrap().to_string(),
        sign1: raw["sign1"].as_str().unwrap().to_string(),
        raw_public_key: raw["raw_public_key"].as_str().unwrap().to_string(),
        raw_signature: raw["raw_signature"].as_str().unwrap().to_string(),
        raw_to_be_signed: raw["raw_to_be_signed"].as_str().unwrap().to_string(),
    }
}

/// Decode a COSE_Sign1 from bytes, stripping CBOR tag 18 if present.
fn decode_sign1(bytes: &[u8]) -> CoseSign1Cbor {
    match from_reader::<CoseSign1Cbor, _>(bytes) {
        Ok(s) => s,
        Err(_) => {
            // Try stripping tag 18
            let value: Value = from_reader(bytes).unwrap();
            if let Value::Tag(18, inner) = value {
                let mut buf = vec![];
                into_writer(&*inner, &mut buf).unwrap();
                from_reader(buf.as_slice()).unwrap()
            } else {
                panic!("expected COSE_Sign1 or tag 18 wrapper");
            }
        }
    }
}

fn hex_decode(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

// ── ML-DSA-44 ──

#[test]
fn ml_dsa_44_decode_cose_key() {
    let tv = load_vector("ML_DSA_44");
    let key_bytes = hex_decode(&tv.key);
    let cose_key: CoseKeyCbor = from_reader(key_bytes.as_slice()).unwrap();

    // Verify kty = 7 (AKP)
    assert_eq!(cose_key.kty, Some(common::TextOrInt::Int(7)));
    // Verify alg = -48 (ML-DSA-44)
    assert_eq!(cose_key.alg, Some(common::TextOrInt::Int(-48)));
}

#[test]
fn ml_dsa_44_decode_sign1() {
    let tv = load_vector("ML_DSA_44");
    let sign1_bytes = hex_decode(&tv.sign1);
    let sign1 = decode_sign1(&sign1_bytes);

    // Verify protected header has alg = -48
    let alg = cose_crypto::helpers::extract_algorithm(&sign1.protected).unwrap();
    assert_eq!(alg, CoseAlgorithm::MlDsa44);
}

#[test]
fn ml_dsa_44_verify_raw() {
    let tv = load_vector("ML_DSA_44");
    let pub_key = hex_decode(&tv.raw_public_key);
    let sig = hex_decode(&tv.raw_signature);
    let tbs = hex_decode(&tv.raw_to_be_signed);
    let verifier = MlDsa44Verifier::from_bytes(&pub_key).unwrap();
    verify_raw(&verifier, &tbs, &sig);
}

#[test]
fn ml_dsa_44_sig_structure_matches_spec() {
    use common::BytesType;
    use cose::arrays::{SigStructure, SigStructureCbor};
    use cose::choices::SignatureOrSignature1;

    let tv = load_vector("ML_DSA_44");
    let sign1_bytes = hex_decode(&tv.sign1);
    let sign1 = decode_sign1(&sign1_bytes);
    let expected_tbs = hex_decode(&tv.raw_to_be_signed);

    let payload = match &sign1.payload {
        common::BinaryOrNil::Binary(p) => p.clone(),
        _ => panic!("no payload"),
    };

    let sig_structure = SigStructure {
        context: SignatureOrSignature1::Signature1,
        body_protected: sign1.protected.clone(),
        sign_protected: None,
        external_aad: BytesType(vec![]),
        payload: BytesType(payload),
    };

    let sig_structure_cbor = SigStructureCbor::try_from(sig_structure).unwrap();
    let mut actual_tbs = Vec::new();
    into_writer(&sig_structure_cbor, &mut actual_tbs).unwrap();

    assert_eq!(
        expected_tbs,
        actual_tbs,
        "Sig_structure mismatch: expected {} bytes, got {} bytes",
        expected_tbs.len(),
        actual_tbs.len()
    );
}

#[test]
fn ml_dsa_44_verify_spec_signature() {
    let tv = load_vector("ML_DSA_44");
    let sign1_bytes = hex_decode(&tv.sign1);
    let sign1 = decode_sign1(&sign1_bytes);

    let pub_key = hex_decode(&tv.raw_public_key);
    let verifier = MlDsa44Verifier::from_bytes(&pub_key).unwrap();

    verify_sign1(&sign1, &verifier, &[]).unwrap();
}

#[test]
fn ml_dsa_44_sign_and_verify() {
    let tv = load_vector("ML_DSA_44");
    let seed = hex_decode(&tv.priv_key);
    let pub_key = hex_decode(&tv.raw_public_key);

    let signer = MlDsa44Signer::from_seed(&seed).unwrap();
    let verifier = MlDsa44Verifier::from_bytes(&pub_key).unwrap();

    // Sign the payload from the spec: "hello post quantum signatures"
    let payload = b"hello post quantum signatures";
    let hdr = cose_crypto::helpers::header_with_algorithm(CoseAlgorithm::MlDsa44);

    let sign1 = cose_crypto::sign::CoseSign1Builder::new()
        .payload(payload)
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    // Verify our own signature
    verify_sign1(&sign1, &verifier, &[]).unwrap();
}

#[test]
fn ml_dsa_44_from_cose_key() {
    let tv = load_vector("ML_DSA_44");
    let key_bytes = hex_decode(&tv.key);
    let cose_key: CoseKeyCbor = from_reader(key_bytes.as_slice()).unwrap();

    // Should be able to create a verifier from the COSE key
    let verifier = MlDsa44Verifier::from_cose_key(&cose_key).unwrap();

    // Verify the spec COSE_Sign1 through the full path
    let sign1_bytes = hex_decode(&tv.sign1);
    let sign1 = decode_sign1(&sign1_bytes);
    verify_sign1(&sign1, &verifier, &[]).unwrap();
}

// ── ML-DSA-65 ──

#[test]
fn ml_dsa_65_decode_cose_key() {
    let tv = load_vector("ML_DSA_65");
    let key_bytes = hex_decode(&tv.key);
    let cose_key: CoseKeyCbor = from_reader(key_bytes.as_slice()).unwrap();

    assert_eq!(cose_key.kty, Some(common::TextOrInt::Int(7)));
    assert_eq!(cose_key.alg, Some(common::TextOrInt::Int(-49)));
}

#[test]
fn ml_dsa_65_verify_spec_signature() {
    let tv = load_vector("ML_DSA_65");
    let sign1_bytes = hex_decode(&tv.sign1);
    let sign1 = decode_sign1(&sign1_bytes);

    let pub_key = hex_decode(&tv.raw_public_key);
    let verifier = MlDsa65Verifier::from_bytes(&pub_key).unwrap();

    verify_sign1(&sign1, &verifier, &[]).unwrap();
}

#[test]
fn ml_dsa_65_sign_and_verify() {
    let tv = load_vector("ML_DSA_65");
    let seed = hex_decode(&tv.priv_key);
    let pub_key = hex_decode(&tv.raw_public_key);

    let signer = MlDsa65Signer::from_seed(&seed).unwrap();
    let verifier = MlDsa65Verifier::from_bytes(&pub_key).unwrap();

    let payload = b"hello post quantum signatures";
    let hdr = cose_crypto::helpers::header_with_algorithm(CoseAlgorithm::MlDsa65);

    let sign1 = cose_crypto::sign::CoseSign1Builder::new()
        .payload(payload)
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    verify_sign1(&sign1, &verifier, &[]).unwrap();
}

// ── ML-DSA-87 ──

#[test]
fn ml_dsa_87_decode_cose_key() {
    let tv = load_vector("ML_DSA_87");
    let key_bytes = hex_decode(&tv.key);
    let cose_key: CoseKeyCbor = from_reader(key_bytes.as_slice()).unwrap();

    assert_eq!(cose_key.kty, Some(common::TextOrInt::Int(7)));
    assert_eq!(cose_key.alg, Some(common::TextOrInt::Int(-50)));
}

#[test]
fn ml_dsa_87_verify_spec_signature() {
    let tv = load_vector("ML_DSA_87");
    let sign1_bytes = hex_decode(&tv.sign1);
    let sign1 = decode_sign1(&sign1_bytes);

    let pub_key = hex_decode(&tv.raw_public_key);
    let verifier = MlDsa87Verifier::from_bytes(&pub_key).unwrap();

    verify_sign1(&sign1, &verifier, &[]).unwrap();
}

#[test]
fn ml_dsa_87_sign_and_verify() {
    let tv = load_vector("ML_DSA_87");
    let seed = hex_decode(&tv.priv_key);
    let pub_key = hex_decode(&tv.raw_public_key);

    let signer = MlDsa87Signer::from_seed(&seed).unwrap();
    let verifier = MlDsa87Verifier::from_bytes(&pub_key).unwrap();

    let payload = b"hello post quantum signatures";
    let hdr = cose_crypto::helpers::header_with_algorithm(CoseAlgorithm::MlDsa87);

    let sign1 = cose_crypto::sign::CoseSign1Builder::new()
        .payload(payload)
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    verify_sign1(&sign1, &verifier, &[]).unwrap();
}
