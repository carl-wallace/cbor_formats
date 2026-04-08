use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use hex_literal::hex;

use common::{BinaryOrNil, BytesType, TextOrInt};
use cose::arrays::CoseSign1Cbor;
use cose::choices::EmptyOrSerializedMap;
use cose::maps::{HeaderMap, HeaderMapCbor};
use coserv::maps::CoservMapCbor;
use coserv::signed::*;

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
        signature: BytesType::Bytes(vec![0x00; 64]),
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
