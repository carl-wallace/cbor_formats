use std::collections::BTreeMap;

use ciborium::{de::from_reader, ser::into_writer, tag::Required};

use cmw::{
    choices::{self, CborCmw, CborCollectionKey, JsonCmw},
    maps::{CborCollection, JsonCollection},
    signed::{CMW_CBOR_CONTENT_FORMAT, CMW_CBOR_CONTENT_TYPE, SignedCborCmw, SignedCmwError},
};
use common::{BinaryOrNil, BytesType, TextOrInt};
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

/// Helper: build a CborCmw::Record from a cbor-record array [type, value, ?ind]
fn make_cbor_record() -> CborCmw {
    // Encode a cbor-record as CBOR array: ["application/eat+cwt", h'A10102', 4]
    let arr = ciborium::Value::Array(vec![
        ciborium::Value::Text("application/eat+cwt".into()),
        ciborium::Value::Bytes(vec![0xA1, 0x01, 0x02]),
        ciborium::Value::Integer(4.into()),
    ]);
    let mut buf = vec![];
    into_writer(&arr, &mut buf).unwrap();
    let cmw: CborCmw = from_reader(buf.as_slice()).unwrap();
    cmw
}

#[test]
fn cbor_cmw_record_roundtrip() {
    let cmw = make_cbor_record();
    roundtrip_cbor(&cmw);
}

#[test]
fn cbor_cmw_tag_cmw_json_collection_data_roundtrip() {
    let cmw =
        CborCmw::TagCmwJsonCollectionData(Required::<BytesType, 1668547093>(BytesType(vec![
            0x01, 0x02, 0x03,
        ])));
    roundtrip_cbor(&cmw);
}

#[test]
fn cbor_cmw_tag_cmw_jws_data_roundtrip() {
    let cmw = CborCmw::TagCmwJwsData(Required::<BytesType, 1668547094>(BytesType(vec![
        0xAA, 0xBB,
    ])));
    roundtrip_cbor(&cmw);
}

#[test]
fn cbor_cmw_tag_signed_roundtrip() {
    // Build a minimal COSE_Sign1 as CBOR array: [protected, unprotected, payload, signature]
    // protected must be a bstr (possibly empty), unprotected is a map
    let sign1_arr = ciborium::Value::Array(vec![
        ciborium::Value::Bytes(vec![]), // protected: empty bstr
        ciborium::Value::Map(vec![]),   // unprotected: empty map
        ciborium::Value::Bytes(b"test payload".to_vec()), // payload: non-nil bstr
        ciborium::Value::Bytes(vec![0x00; 32]), // signature: 32 bytes
    ]);
    // Wrap in tag 1668547092
    let tagged = ciborium::Value::Tag(1668547092, Box::new(sign1_arr));
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();

    let cmw: CborCmw = from_reader(buf.as_slice()).unwrap();
    assert!(matches!(cmw, CborCmw::TagSigned(_)));
    roundtrip_cbor(&cmw);
}

#[test]
fn cbor_collection_roundtrip() {
    let inner =
        CborCmw::TagCmwJsonCollectionData(Required::<BytesType, 1668547093>(BytesType(vec![0x01])));

    let mut entries = BTreeMap::new();
    entries.insert(CborCollectionKey::Int(1), inner.clone());
    entries.insert(CborCollectionKey::Text("foo".into()), inner);

    let col = CborCollection {
        collection_type: None,
        entries,
    };
    roundtrip_cbor(&col);
}

#[test]
fn cbor_collection_with_type_roundtrip() {
    let inner =
        CborCmw::TagCmwJsonCollectionData(Required::<BytesType, 1668547093>(BytesType(vec![0xFF])));

    let mut entries = BTreeMap::new();
    entries.insert(CborCollectionKey::Int(0), inner);

    let col = CborCollection {
        collection_type: Some("urn:example:collection".into()),
        entries,
    };
    roundtrip_cbor(&col);
}

#[test]
fn cbor_cmw_tag_collection_roundtrip() {
    let inner = CborCmw::TagCmwJwsData(Required::<BytesType, 1668547094>(BytesType(vec![
        0xDE, 0xAD,
    ])));

    let mut entries = BTreeMap::new();
    entries.insert(CborCollectionKey::Text("a".into()), inner);

    let col = CborCollection {
        collection_type: None,
        entries,
    };
    let cmw = CborCmw::TagCollection(Required::<Box<CborCollection>, 1668547091>(Box::new(col)));
    roundtrip_cbor(&cmw);
}

#[test]
fn json_collection_roundtrip() {
    // Build a json-record as CBOR array: ["application/json", "{}", null]
    let record_arr = ciborium::Value::Array(vec![
        ciborium::Value::Text("application/json".into()),
        ciborium::Value::Text(r#"{"hello":"world"}"#.into()),
    ]);
    let mut record_buf = vec![];
    into_writer(&record_arr, &mut record_buf).unwrap();
    let inner: JsonCmw = from_reader(record_buf.as_slice()).unwrap();

    let mut entries = BTreeMap::new();
    entries.insert("entry1".into(), inner);

    let col = JsonCollection {
        collection_type: Some("urn:example:json-col".into()),
        entries,
    };
    roundtrip_cbor(&col);
}

#[test]
fn empty_cbor_collection_rejected() {
    let empty_map = ciborium::Value::Map(vec![]);
    let mut buf = vec![];
    into_writer(&empty_map, &mut buf).unwrap();

    let result = from_reader::<CborCollection, _>(buf.as_slice());
    assert!(result.is_err(), "empty cbor-collection should be rejected");
}

#[test]
fn empty_json_collection_rejected() {
    let empty_map = ciborium::Value::Map(vec![]);
    let mut buf = vec![];
    into_writer(&empty_map, &mut buf).unwrap();

    let result = from_reader::<JsonCollection, _>(buf.as_slice());
    assert!(result.is_err(), "empty json-collection should be rejected");
}

#[test]
fn cmwc_t_only_collection_rejected() {
    // A collection with only __cmwc_t and no entries should be rejected
    let map = ciborium::Value::Map(vec![(
        ciborium::Value::Text("__cmwc_t".into()),
        ciborium::Value::Text("urn:example".into()),
    )]);
    let mut buf = vec![];
    into_writer(&map, &mut buf).unwrap();

    let result = from_reader::<CborCollection, _>(buf.as_slice());
    assert!(
        result.is_err(),
        "collection with only __cmwc_t should be rejected"
    );
}

#[test]
fn cbor_cmw_tag_data_roundtrip() {
    // Use an arbitrary tag in the CMW range (TN(42) = CMW_TAG_MIN + 42*256 + 42 ... actually
    // just pick a value in range that doesn't collide with the well-known tags)
    let tag = choices::CMW_TAG_MIN + 100;
    let cmw = CborCmw::TagData {
        tag,
        value: BytesType(vec![0xCA, 0xFE, 0xBA, 0xBE]),
    };
    let buf = roundtrip_cbor(&cmw);

    // Verify the raw CBOR is a tagged byte string
    let raw: ciborium::Value = from_reader(buf.as_slice()).unwrap();
    match raw {
        ciborium::Value::Tag(t, inner) => {
            assert_eq!(t, tag);
            assert!(matches!(*inner, ciborium::Value::Bytes(_)));
        }
        other => panic!("expected Tag, got {other:?}"),
    }
}

#[test]
fn cbor_cmw_tag_data_at_range_boundaries() {
    for tag in [choices::CMW_TAG_MIN, choices::CMW_TAG_MAX] {
        let cmw = CborCmw::TagData {
            tag,
            value: BytesType(vec![0x00]),
        };
        roundtrip_cbor(&cmw);
    }
}

#[test]
fn cbor_cmw_tag_outside_range_rejected() {
    // Tag just below the CMW range
    let tagged = ciborium::Value::Tag(
        choices::CMW_TAG_MIN - 1,
        Box::new(ciborium::Value::Bytes(vec![0x00])),
    );
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let result = from_reader::<CborCmw, _>(buf.as_slice());
    assert!(result.is_err(), "tag below CMW range should be rejected");

    // Tag just above the CMW range
    let tagged = ciborium::Value::Tag(
        choices::CMW_TAG_MAX + 1,
        Box::new(ciborium::Value::Bytes(vec![0x00])),
    );
    buf.clear();
    into_writer(&tagged, &mut buf).unwrap();
    let result = from_reader::<CborCmw, _>(buf.as_slice());
    assert!(result.is_err(), "tag above CMW range should be rejected");
}

#[test]
fn cbor_cmw_tag_data_in_collection() {
    let tag_data = CborCmw::TagData {
        tag: choices::CMW_TAG_MIN + 500,
        value: BytesType(vec![0x01, 0x02]),
    };

    let mut entries = BTreeMap::new();
    entries.insert(CborCollectionKey::Text("custom".into()), tag_data);

    let col = CborCollection {
        collection_type: None,
        entries,
    };
    roundtrip_cbor(&col);
}

// ---------- SignedCborCmw tests ----------

/// Helper: serialize a HeaderMap to EmptyOrSerializedMap.
fn serialize_header(hdr: &HeaderMap) -> EmptyOrSerializedMap {
    let cbor = HeaderMapCbor::try_from(hdr).unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    EmptyOrSerializedMap::SerializedMap(buf)
}

/// Helper: build a valid signed-cbor-cmw CoseSign1Cbor.
fn make_valid_signed_cmw() -> CoseSign1Cbor {
    // Build a cbor-record payload
    let cmw = make_cbor_record();
    let mut payload_buf = vec![];
    into_writer(&cmw, &mut payload_buf).unwrap();

    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)), // ES256
        criticality: None,
        content_type: Some(TextOrInt::Text(CMW_CBOR_CONTENT_TYPE.to_string())),
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
        payload: BinaryOrNil::Binary(payload_buf),
        signature: BytesType(vec![0x00; 64]),
    }
}

#[test]
fn signed_cbor_cmw_valid() {
    let sign1 = make_valid_signed_cmw();
    let signed = SignedCborCmw::new(sign1).expect("should validate");
    let cmw = signed.payload().expect("should decode payload");
    assert!(matches!(cmw, CborCmw::Record(_)));
}

#[test]
fn signed_cbor_cmw_roundtrip() {
    let sign1 = make_valid_signed_cmw();
    let signed = SignedCborCmw::new(sign1).unwrap();
    roundtrip_cbor(&signed);
}

#[test]
fn signed_cbor_cmw_content_format_number() {
    let mut sign1 = make_valid_signed_cmw();
    // Use integer content format 10000 instead of text
    let hdr = HeaderMap {
        alg_id: Some(TextOrInt::Int(-7)),
        criticality: None,
        content_type: Some(TextOrInt::Int(CMW_CBOR_CONTENT_FORMAT)),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    sign1.protected = serialize_header(&hdr);
    SignedCborCmw::new(sign1).expect("integer content format should be accepted");
}

#[test]
fn signed_cbor_cmw_missing_alg_rejected() {
    let mut sign1 = make_valid_signed_cmw();
    let hdr = HeaderMap {
        alg_id: None,
        criticality: None,
        content_type: Some(TextOrInt::Text(CMW_CBOR_CONTENT_TYPE.to_string())),
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    sign1.protected = serialize_header(&hdr);
    let err = SignedCborCmw::new(sign1).unwrap_err();
    assert_eq!(err, SignedCmwError::MissingAlgorithm);
}

#[test]
fn signed_cbor_cmw_wrong_content_type_rejected() {
    let mut sign1 = make_valid_signed_cmw();
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
    let err = SignedCborCmw::new(sign1).unwrap_err();
    assert_eq!(err, SignedCmwError::InvalidContentType);
}

#[test]
fn signed_cbor_cmw_missing_content_type_rejected() {
    let mut sign1 = make_valid_signed_cmw();
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
    let err = SignedCborCmw::new(sign1).unwrap_err();
    assert_eq!(err, SignedCmwError::InvalidContentType);
}

#[test]
fn signed_cbor_cmw_empty_protected_rejected() {
    let mut sign1 = make_valid_signed_cmw();
    sign1.protected = EmptyOrSerializedMap::Empty(vec![]);
    let err = SignedCborCmw::new(sign1).unwrap_err();
    assert_eq!(err, SignedCmwError::EmptyProtectedHeader);
}

#[test]
fn signed_cbor_cmw_nil_payload_rejected() {
    let mut sign1 = make_valid_signed_cmw();
    sign1.payload = BinaryOrNil::Nil;
    let err = SignedCborCmw::new(sign1).unwrap_err();
    assert_eq!(err, SignedCmwError::MissingPayload);
}

#[test]
fn signed_cbor_cmw_invalid_payload_rejected() {
    let mut sign1 = make_valid_signed_cmw();
    // Set payload to bytes that are not valid cbor-cmw
    sign1.payload = BinaryOrNil::Binary(vec![0xFF, 0xFF]);
    let err = SignedCborCmw::new(sign1).unwrap_err();
    assert!(matches!(err, SignedCmwError::InvalidPayload(_)));
}
