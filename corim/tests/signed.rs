use ciborium::{de::from_reader, ser::into_writer, tag::Required, value::Value};

use common::{BinaryOrNil, BytesType, TextOrInt};
use corim::{
    TaggedUnsignedCorimMap,
    choices::CorimIdTypeChoice,
    maps::{ComidEntityMapCbor, CorimEntityMapCbor, CorimMapCbor},
    signed::{CORIM_CBOR_CONTENT_TYPE, SignedCorim, SignedCorimError},
};
use cose::{
    arrays::CoseSign1Cbor,
    choices::EmptyOrSerializedMap,
    maps::{HeaderMap, HeaderMapCbor},
};

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

fn serialize_header(hdr: &HeaderMap) -> EmptyOrSerializedMap {
    let cbor = HeaderMapCbor::try_from(hdr).unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    EmptyOrSerializedMap::SerializedMap(buf)
}

/// Build a minimal CorimMapCbor for testing.
fn make_corim_map() -> CorimMapCbor {
    CorimMapCbor {
        id: CorimIdTypeChoice::Str("test-corim-id".into()),
        tags: vec![BytesType(vec![0xA0])], // empty map as placeholder
        dependent_rims: None,
        profile: None,
        rim_validity: None,
        entities: None,
    }
}

/// Serialize a CorimMapCbor as tagged payload (tag 501).
fn make_tagged_payload_bytes(corim: &CorimMapCbor) -> Vec<u8> {
    let mut inner_buf = vec![];
    into_writer(corim, &mut inner_buf).unwrap();
    let inner_value: Value = from_reader(inner_buf.as_slice()).unwrap();
    let tagged = Value::Tag(501, Box::new(inner_value));
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    buf
}

/// Serialize a CorimMapCbor as bare (untagged) payload.
fn make_bare_payload_bytes(corim: &CorimMapCbor) -> Vec<u8> {
    let mut buf = vec![];
    into_writer(corim, &mut buf).unwrap();
    buf
}

fn make_valid_signed_corim() -> CoseSign1Cbor {
    let corim = make_corim_map();
    let payload_bytes = make_tagged_payload_bytes(&corim);

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)), // ES256
        criticality: None,
        content_type: Some(TextOrInt::Text(CORIM_CBOR_CONTENT_TYPE.to_string())),
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

// -- Type alias tests --

#[test]
fn corim_entity_map_alias() {
    // CorimEntityMapCbor and ComidEntityMapCbor should be the same type
    let _: CorimEntityMapCbor;
    let _: ComidEntityMapCbor;
}

#[test]
fn tagged_unsigned_corim_map_roundtrip() {
    let corim = make_corim_map();
    let tagged: TaggedUnsignedCorimMap = Required(corim);
    roundtrip_cbor(&tagged);
}

// -- SignedCorim tests --

#[test]
fn signed_corim_valid() {
    let sign1 = make_valid_signed_corim();
    let signed = SignedCorim::new(sign1).expect("should validate");
    let _payload = signed.payload().expect("should decode payload");
}

#[test]
fn signed_corim_roundtrip() {
    let sign1 = make_valid_signed_corim();
    let signed = SignedCorim::new(sign1).unwrap();
    roundtrip_cbor(&signed);
}

#[test]
fn signed_corim_bare_payload_accepted() {
    let corim = make_corim_map();
    let payload_bytes = make_bare_payload_bytes(&corim);
    let mut sign1 = make_valid_signed_corim();
    sign1.payload = BinaryOrNil::Binary(payload_bytes);
    let signed = SignedCorim::new(sign1).expect("bare payload should be accepted");
    let _payload = signed.payload().expect("should decode bare payload");
}

#[test]
fn signed_corim_missing_alg_rejected() {
    let mut sign1 = make_valid_signed_corim();
    let hdr = HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: Some(TextOrInt::Text(CORIM_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    sign1.protected = serialize_header(&hdr);
    let err = SignedCorim::new(sign1).unwrap_err();
    assert_eq!(err, SignedCorimError::MissingAlgorithm);
}

#[test]
fn signed_corim_wrong_content_type_rejected() {
    let mut sign1 = make_valid_signed_corim();
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
    let err = SignedCorim::new(sign1).unwrap_err();
    assert_eq!(err, SignedCorimError::InvalidContentType);
}

#[test]
fn signed_corim_missing_content_type_rejected() {
    let mut sign1 = make_valid_signed_corim();
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
    let err = SignedCorim::new(sign1).unwrap_err();
    assert_eq!(err, SignedCorimError::InvalidContentType);
}

#[test]
fn signed_corim_empty_protected_rejected() {
    let mut sign1 = make_valid_signed_corim();
    sign1.protected = EmptyOrSerializedMap::Empty(vec![]);
    let err = SignedCorim::new(sign1).unwrap_err();
    assert_eq!(err, SignedCorimError::EmptyProtectedHeader);
}

#[test]
fn signed_corim_nil_payload_rejected() {
    let mut sign1 = make_valid_signed_corim();
    sign1.payload = BinaryOrNil::Nil;
    let err = SignedCorim::new(sign1).unwrap_err();
    assert_eq!(err, SignedCorimError::MissingPayload);
}

#[test]
fn signed_corim_invalid_payload_rejected() {
    let mut sign1 = make_valid_signed_corim();
    sign1.payload = BinaryOrNil::Binary(vec![0xFF, 0xFF]);
    let err = SignedCorim::new(sign1).unwrap_err();
    assert!(matches!(err, SignedCorimError::InvalidPayload(_)));
}

#[test]
fn veraison_signed_corim_decode() {
    let data = std::fs::read("./tests/examples/signed-corim-veraison.cbor")
        .expect("test vector file missing");

    // Should decode as CoseSign1Cbor (possibly after stripping tag 18)
    let sign1: CoseSign1Cbor = match from_reader(data.as_slice()) {
        Ok(s) => s,
        Err(_) => {
            // Try stripping outer tag 18 (signed-corim = #6.18(...))
            let value: Value = from_reader(data.as_slice()).unwrap();
            if let Value::Tag(18, inner) = value {
                let mut buf = vec![];
                into_writer(&*inner, &mut buf).unwrap();
                from_reader(buf.as_slice()).unwrap()
            } else {
                panic!("expected tag 18 wrapper");
            }
        }
    };

    // Roundtrip the CoseSign1Cbor
    let mut buf = vec![];
    into_writer(&sign1, &mut buf).unwrap();
    let sign1_rt: CoseSign1Cbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(sign1, sign1_rt);
}
