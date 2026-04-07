use ciborium::ser::into_writer;
use ciborium::value::Value;

use cose::choices::*;

#[test]
fn empty_or_serialized_map_empty() {
    let value = Value::Bytes(vec![]);
    let result = EmptyOrSerializedMap::try_from(&value);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), EmptyOrSerializedMap::Empty(_)));
}

#[test]
fn empty_or_serialized_map_valid_header() {
    // Encode a valid header_map: {1: -7} (alg_id = ES256)
    use common::TextOrInt;
    use cose::maps::*;
    let hm = HeaderMapCbor {
        alg_id: Some(TextOrInt::Int(-7)),
        criticality: None,
        content_type: None,
        key_id: None,
        iv: None,
        partial_iv: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&hm, &mut buf);

    let value = Value::Bytes(buf);
    let result = EmptyOrSerializedMap::try_from(&value);
    assert!(result.is_ok());
    assert!(matches!(
        result.unwrap(),
        EmptyOrSerializedMap::SerializedMap(_)
    ));
}

#[test]
fn empty_or_serialized_map_invalid_bytes() {
    // Invalid CBOR bytes that don't decode as a header_map
    let value = Value::Bytes(vec![0xFF, 0xFF]);
    let result = EmptyOrSerializedMap::try_from(&value);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid header_map"));
}

#[test]
fn empty_or_serialized_map_non_bytes_fails() {
    let value = Value::Text("not bytes".to_string());
    let result = EmptyOrSerializedMap::try_from(&value);
    assert!(result.is_err());
}
