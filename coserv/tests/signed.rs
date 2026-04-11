use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use hex_literal::hex;

use common::{BinaryOrNil, BytesType, TextOrInt};
use cose::arrays::CoseSign1Cbor;
use cose::choices::EmptyOrSerializedMap;
use cose::maps::{HeaderMap, HeaderMapCbor};
use cose_crypto::algorithm::CoseAlgorithm;
use cose_crypto::crypto::ecdsa::{Es256Signer, Es256Verifier};
use cose_crypto::sign::{CoseSign1Builder, verify_sign1};
use coserv::maps::CoservMapCbor;
use coserv::signed::*;
use p256::elliptic_curve::Generate;

fn roundtrip_cbor<
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
>(
    val: &T,
) -> Vec<u8> {
    let mut buf = vec![];
    into_writer(val, &mut buf).expect("serialize");
    let decoded: T = from_reader(buf.as_slice()).expect("deserialize");
    assert_eq!(*val, decoded);
    buf
}

/// Serialize a HeaderMap to EmptyOrSerializedMap.
fn serialize_header(hdr: &HeaderMap) -> EmptyOrSerializedMap {
    let cbor = HeaderMapCbor::try_from(hdr).unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    EmptyOrSerializedMap::SerializedMap(buf)
}

/// Deserialize a valid CoservMapCbor from test hex data and serialize it to
/// bytes suitable for use as a COSE_Sign1 payload.
fn make_coserv_payload_bytes() -> Vec<u8> {
    // Minimal coserv: {0: "tag:example.com,2025:cc-platform#1.0.0", 1: {0: 2, 1: {0: [[...]]}, 2: 0}}
    let coserv_cbor = hex!(
        "a20078267461673a6578616d706c652e636f6d2c323032353a63632d706c6174666f726d23312e302e30"
        "01a3000201a1008281a300d902304400112233016e4578616d706c652056656e646f72026d4578616d70"
        "6c65204d6f64656c81a100d8255031fb5abf023e4992aa4e95f9c1503bfa0200"
    );
    // Verify it decodes
    let _: CoservMapCbor = from_reader(coserv_cbor.as_slice()).unwrap();
    coserv_cbor.to_vec()
}

/// Build a valid signed-coserv CoseSign1Cbor.
fn make_valid_signed_coserv() -> CoseSign1Cbor {
    let payload_bytes = make_coserv_payload_bytes();

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)), // ES256
        criticality: None,
        content_type: Some(TextOrInt::Text(COSERV_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    CoseSign1Cbor {
        protected: serialize_header(&hdr),
        unprotected: HeaderMapCbor {
            alg_id: None,
            criticality: None,
            content_type: None,
            key_id: None,
            iv: None,
            partial_iv: None,
            other: None,
        },
        payload: BinaryOrNil::Binary(payload_bytes),
        signature: BytesType(vec![0x00; 64]),
    }
}

#[test]
fn signed_coserv_valid() {
    let sign1 = make_valid_signed_coserv();
    let signed = SignedCoserv::new(sign1).expect("should validate");
    let _payload = signed.payload().expect("should decode payload");
}

#[test]
fn signed_coserv_roundtrip() {
    let sign1 = make_valid_signed_coserv();
    let signed = SignedCoserv::new(sign1).unwrap();
    roundtrip_cbor(&signed);
}

#[test]
fn signed_coserv_missing_alg_rejected() {
    let mut sign1 = make_valid_signed_coserv();
    let hdr = HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: Some(TextOrInt::Text(COSERV_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    sign1.protected = serialize_header(&hdr);
    let err = SignedCoserv::new(sign1).unwrap_err();
    assert_eq!(err, SignedCoservError::MissingAlgorithm);
}

#[test]
fn signed_coserv_wrong_content_type_rejected() {
    let mut sign1 = make_valid_signed_coserv();
    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)),
        criticality: None,
        content_type: Some(TextOrInt::Text("application/json".to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    sign1.protected = serialize_header(&hdr);
    let err = SignedCoserv::new(sign1).unwrap_err();
    assert_eq!(err, SignedCoservError::InvalidContentType);
}

#[test]
fn signed_coserv_missing_content_type_rejected() {
    let mut sign1 = make_valid_signed_coserv();
    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)),
        criticality: None,
        content_type: None,
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    sign1.protected = serialize_header(&hdr);
    let err = SignedCoserv::new(sign1).unwrap_err();
    assert_eq!(err, SignedCoservError::InvalidContentType);
}

#[test]
fn signed_coserv_empty_protected_rejected() {
    let mut sign1 = make_valid_signed_coserv();
    sign1.protected = EmptyOrSerializedMap::Empty(vec![]);
    let err = SignedCoserv::new(sign1).unwrap_err();
    assert_eq!(err, SignedCoservError::EmptyProtectedHeader);
}

#[test]
fn signed_coserv_nil_payload_rejected() {
    let mut sign1 = make_valid_signed_coserv();
    sign1.payload = BinaryOrNil::Nil;
    let err = SignedCoserv::new(sign1).unwrap_err();
    assert_eq!(err, SignedCoservError::MissingPayload);
}

#[test]
fn signed_coserv_invalid_payload_rejected() {
    let mut sign1 = make_valid_signed_coserv();
    sign1.payload = BinaryOrNil::Binary(vec![0xFF, 0xFF]);
    let err = SignedCoserv::new(sign1).unwrap_err();
    assert!(matches!(err, SignedCoservError::InvalidPayload(_)));
}

// ── Crypto sign/verify round-trip tests ──

/// Generate a P-256 key pair and return (signer, verifier).
fn make_es256_key_pair() -> (Es256Signer, Es256Verifier) {
    let signing_key = p256::ecdsa::SigningKey::generate();
    let d = signing_key.to_bytes();
    let verifying_key = signing_key.verifying_key();
    let point = verifying_key.to_sec1_point(false);
    let x = point.x().unwrap().to_vec();
    let y = point.y().unwrap().to_vec();

    let signer = Es256Signer::from_bytes(&d).unwrap();
    let verifier = Es256Verifier::from_xy(&x, &y).unwrap();
    (signer, verifier)
}

#[test]
fn signed_coserv_sign_verify_round_trip() {
    let (signer, verifier) = make_es256_key_pair();
    let payload_bytes = make_coserv_payload_bytes();

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(CoseAlgorithm::Es256.to_i64())),
        criticality: None,
        content_type: Some(TextOrInt::Text(COSERV_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    let sign1 = CoseSign1Builder::new()
        .payload(&payload_bytes)
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    // Verify the signature
    verify_sign1(&sign1, &verifier, &[]).unwrap();

    // Validate as a SignedCoserv
    let signed = SignedCoserv::new(sign1).expect("should validate as signed-coserv");
    let decoded_payload = signed.payload().expect("should decode payload");

    // Verify the decoded payload matches what we encoded
    let mut expected_buf = vec![];
    into_writer(&decoded_payload, &mut expected_buf).unwrap();
    assert_eq!(payload_bytes, expected_buf);
}

#[test]
fn signed_coserv_wrong_key_fails() {
    let (signer, _verifier) = make_es256_key_pair();
    let (_wrong_signer, wrong_verifier) = make_es256_key_pair();
    let payload_bytes = make_coserv_payload_bytes();

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(CoseAlgorithm::Es256.to_i64())),
        criticality: None,
        content_type: Some(TextOrInt::Text(COSERV_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    let sign1 = CoseSign1Builder::new()
        .payload(&payload_bytes)
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    // Verification with wrong key should fail
    assert!(verify_sign1(&sign1, &wrong_verifier, &[]).is_err());
}

#[test]
fn signed_coserv_cbor_serialization_round_trip() {
    let (signer, verifier) = make_es256_key_pair();
    let payload_bytes = make_coserv_payload_bytes();

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(CoseAlgorithm::Es256.to_i64())),
        criticality: None,
        content_type: Some(TextOrInt::Text(COSERV_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };

    let sign1 = CoseSign1Builder::new()
        .payload(&payload_bytes)
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    // Serialize to CBOR and back
    let mut cbor_bytes = Vec::new();
    into_writer(&sign1, &mut cbor_bytes).unwrap();
    let sign1_rt: CoseSign1Cbor = from_reader(cbor_bytes.as_slice()).unwrap();

    // Verify the deserialized message still validates
    verify_sign1(&sign1_rt, &verifier, &[]).unwrap();

    // And it's still a valid SignedCoserv
    SignedCoserv::new(sign1_rt).expect("deserialized should validate");
}

#[cfg(feature = "crypto")]
#[test]
fn signed_coserv_builder_round_trip() {
    let (signer, verifier) = make_es256_key_pair();

    // Decode the test payload into a CoservMapCbor
    let payload_bytes = make_coserv_payload_bytes();
    let coserv_map: CoservMapCbor = from_reader(payload_bytes.as_slice()).unwrap();

    let hdr = cose_crypto::helpers::header_with_algorithm(CoseAlgorithm::Es256);

    let signed = coserv::signed::SignedCoservBuilder::new()
        .payload(&coserv_map)
        .unwrap()
        .protected(hdr)
        .sign(&signer)
        .unwrap();

    // Verify the signature on the inner CoseSign1
    verify_sign1(signed.as_inner(), &verifier, &[]).unwrap();

    // Verify payload decodes correctly
    let decoded_payload = signed.payload().unwrap();
    let mut rt_buf = vec![];
    into_writer(&decoded_payload, &mut rt_buf).unwrap();
    assert_eq!(payload_bytes, rt_buf);
}
