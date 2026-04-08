use std::collections::BTreeMap;

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::tag::Required;

use cmw::choices::*;
use cmw::maps::*;
use common::BytesType;

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
    let cmw = CborCmw::TagCmwJsonCollectionData(Required::<BytesType, 1668547093>(
        BytesType::Bytes(vec![0x01, 0x02, 0x03]),
    ));
    roundtrip_cbor(&cmw);
}

#[test]
fn cbor_cmw_tag_cmw_jws_data_roundtrip() {
    let cmw = CborCmw::TagCmwJwsData(Required::<BytesType, 1668547094>(BytesType::Bytes(vec![
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
    let inner = CborCmw::TagCmwJsonCollectionData(Required::<BytesType, 1668547093>(
        BytesType::Bytes(vec![0x01]),
    ));

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
    let inner = CborCmw::TagCmwJsonCollectionData(Required::<BytesType, 1668547093>(
        BytesType::Bytes(vec![0xFF]),
    ));

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
    let inner = CborCmw::TagCmwJwsData(Required::<BytesType, 1668547094>(BytesType::Bytes(vec![
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
