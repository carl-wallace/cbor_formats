// Imports required by generated code from the derive macros

use core::{fmt, marker::PhantomData};
use std::collections::BTreeMap;

use ciborium::{cbor, de::from_reader, ser::into_writer, value::Value};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{Error, MapAccess, Visitor},
    ser::Error as OtherError,
};

use cbor_derive::{StructToArray, StructToMap, StructToOneOrMore};
use common::tuple::TupleCbor;

// --- StructToArray tests ---

#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SimpleArray {
    #[cbor(value = "Integer")]
    pub id: u64,
    #[cbor(value = "Text")]
    pub name: String,
}

#[test]
fn struct_to_array_basic_roundtrip() {
    let orig = SimpleArrayCbor {
        id: 42,
        name: "test".to_string(),
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: SimpleArrayCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

#[test]
fn struct_to_array_try_from_value() {
    let orig = SimpleArrayCbor {
        id: 1,
        name: "hello".to_string(),
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();

    let value: Value = from_reader(buf.as_slice()).unwrap();
    let from_ref = SimpleArrayCbor::try_from(&value).unwrap();
    let from_owned = SimpleArrayCbor::try_from(value).unwrap();
    assert_eq!(orig, from_ref);
    assert_eq!(orig, from_owned);
}

#[test]
fn struct_to_array_try_from_vec_value() {
    let orig = SimpleArrayCbor {
        id: 99,
        name: "vec-test".to_string(),
    };
    let vec_val: Vec<Value> = Vec::<Value>::try_from(&orig).unwrap();
    let roundtrip = SimpleArrayCbor::try_from(vec_val).unwrap();
    assert_eq!(orig, roundtrip);
}

#[test]
fn struct_to_array_json_conversion() {
    let cbor_val = SimpleArrayCbor {
        id: 7,
        name: "json-test".to_string(),
    };
    let json: SimpleArray = SimpleArray::try_from(&cbor_val).unwrap();
    assert_eq!(json.id, 7);
    assert_eq!(json.name, "json-test");

    let back: SimpleArrayCbor = json.try_into().unwrap();
    assert_eq!(cbor_val, back);
}

// --- StructToArray with optional fields ---

#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ArrayWithOptional {
    #[cbor(value = "Text")]
    pub required_field: String,
    #[cbor(value = "Text")]
    pub optional_field: Option<String>,
}

#[test]
fn struct_to_array_with_optional_present() {
    let orig = ArrayWithOptionalCbor {
        required_field: "required".to_string(),
        optional_field: Some("optional".to_string()),
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: ArrayWithOptionalCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

#[test]
fn struct_to_array_with_optional_absent() {
    let orig = ArrayWithOptionalCbor {
        required_field: "required".to_string(),
        optional_field: None,
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: ArrayWithOptionalCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

// --- StructToArray with bytes ---

#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ArrayWithBytes {
    #[cbor(value = "Integer")]
    pub alg: u64,
    #[cbor(value = "Bytes")]
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[test]
fn struct_to_array_with_bytes() {
    let orig = ArrayWithBytesCbor {
        alg: 1,
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: ArrayWithBytesCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);

    let json: ArrayWithBytes = decoded.try_into().unwrap();
    assert_eq!(json.data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
}

// --- StructToMap tests ---

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SimpleMap {
    #[cbor(tag = "0", value = "Text")]
    pub name: String,
    #[cbor(tag = "1", value = "Integer")]
    pub count: u64,
}

#[test]
fn struct_to_map_basic_roundtrip() {
    let orig = SimpleMapCbor {
        name: "test".to_string(),
        count: 42,
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: SimpleMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

#[test]
fn struct_to_map_try_from_value() {
    let orig = SimpleMapCbor {
        name: "value-test".to_string(),
        count: 100,
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();

    let value: Value = from_reader(buf.as_slice()).unwrap();
    let from_ref = SimpleMapCbor::try_from(&value).unwrap();
    let from_owned = SimpleMapCbor::try_from(value).unwrap();
    assert_eq!(orig, from_ref);
    assert_eq!(orig, from_owned);
}

#[test]
fn struct_to_map_json_conversion() {
    let cbor_val = SimpleMapCbor {
        name: "json-test".to_string(),
        count: 7,
    };
    let json: SimpleMap = SimpleMap::try_from(&cbor_val).unwrap();
    assert_eq!(json.name, "json-test");
    assert_eq!(json.count, 7);

    let back: SimpleMapCbor = json.try_into().unwrap();
    assert_eq!(cbor_val, back);
}

// --- StructToMap with optional fields ---

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MapWithOptional {
    #[cbor(tag = "0", value = "Text")]
    pub required_name: String,
    #[cbor(tag = "1", value = "Integer")]
    pub optional_count: Option<u64>,
    #[cbor(tag = "2", value = "Bool")]
    pub optional_flag: Option<bool>,
}

#[test]
fn struct_to_map_optional_all_present() {
    let orig = MapWithOptionalCbor {
        required_name: "test".to_string(),
        optional_count: Some(42),
        optional_flag: Some(true),
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: MapWithOptionalCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);

    let json: MapWithOptional = decoded.try_into().unwrap();
    assert_eq!(json.optional_count, Some(42));
    assert_eq!(json.optional_flag, Some(true));
}

#[test]
fn struct_to_map_optional_none() {
    let orig = MapWithOptionalCbor {
        required_name: "minimal".to_string(),
        optional_count: None,
        optional_flag: None,
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: MapWithOptionalCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

// --- StructToMap with nested cbor types ---

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct InnerMap {
    #[cbor(tag = "0", value = "Text")]
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct OuterMap {
    #[cbor(tag = "0", value = "Text")]
    pub label: String,
    #[cbor(tag = "1", value = "Map", cbor = "true")]
    pub inner: InnerMap,
}

#[test]
fn struct_to_map_nested() {
    let orig = OuterMapCbor {
        label: "outer".to_string(),
        inner: InnerMapCbor {
            data: "inner-data".to_string(),
        },
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: OuterMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);

    let json: OuterMap = decoded.try_into().unwrap();
    assert_eq!(json.inner.data, "inner-data");
    let back: OuterMapCbor = json.try_into().unwrap();
    assert_eq!(orig, back);
}

// --- StructToOneOrMore ---

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, StructToOneOrMore, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct OneOrMoreItem {
    #[cbor(tag = "0", value = "Text")]
    pub value: String,
}

#[test]
fn struct_to_one_or_more_single() {
    let single = OneOrMoreOneOrMoreItemCbor::One(OneOrMoreItemCbor {
        value: "single".to_string(),
    });
    let mut buf = vec![];
    into_writer(&single, &mut buf).unwrap();
    let decoded: OneOrMoreOneOrMoreItemCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(single, decoded);
}

#[test]
fn struct_to_one_or_more_multiple() {
    let items = vec![
        OneOrMoreItemCbor {
            value: "first".to_string(),
        },
        OneOrMoreItemCbor {
            value: "second".to_string(),
        },
    ];
    let multiple = OneOrMoreOneOrMoreItemCbor::More(items);
    let mut buf = vec![];
    into_writer(&multiple, &mut buf).unwrap();
    let decoded: OneOrMoreOneOrMoreItemCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(multiple, decoded);
}

#[test]
fn struct_to_one_or_more_json_conversion() {
    let cbor_single = OneOrMoreOneOrMoreItemCbor::One(OneOrMoreItemCbor {
        value: "test".to_string(),
    });
    let json: OneOrMoreOneOrMoreItem = OneOrMoreOneOrMoreItem::try_from(&cbor_single).unwrap();
    let back: OneOrMoreOneOrMoreItemCbor = json.try_into().unwrap();
    assert_eq!(cbor_single, back);
}

// --- StructToMap with Vec field ---

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MapWithVec {
    #[cbor(tag = "0", value = "Text")]
    pub name: String,
    #[cbor(tag = "1", value = "Array", cbor = "true")]
    pub items: Vec<SimpleArray>,
}

#[test]
fn struct_to_map_with_vec_field() {
    let orig = MapWithVecCbor {
        name: "vec-map".to_string(),
        items: vec![
            SimpleArrayCbor {
                id: 1,
                name: "a".to_string(),
            },
            SimpleArrayCbor {
                id: 2,
                name: "b".to_string(),
            },
        ],
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: MapWithVecCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);

    let json: MapWithVec = decoded.try_into().unwrap();
    assert_eq!(json.items.len(), 2);
    let back: MapWithVecCbor = json.try_into().unwrap();
    assert_eq!(orig, back);
}

// --- non-empty constraint tests (map) ---

#[derive(Clone, Debug, PartialEq, Eq, StructToMap, Serialize, Deserialize)]
#[cbor(non_empty = "true")]
pub struct NonEmptyMap {
    #[cbor(tag = "0", value = "Text")]
    pub field_a: Option<String>,
    #[cbor(tag = "1", value = "Integer")]
    pub field_b: Option<u64>,
}

#[test]
fn non_empty_map_serialize_with_field_present() {
    let orig = NonEmptyMapCbor {
        field_a: Some("hello".to_string()),
        field_b: None,
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: NonEmptyMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

#[test]
fn non_empty_map_serialize_all_none_fails() {
    let empty = NonEmptyMapCbor {
        field_a: None,
        field_b: None,
    };
    let mut buf = vec![];
    let result = into_writer(&empty, &mut buf);
    assert!(result.is_err());
}

#[test]
fn non_empty_map_deserialize_empty_fails() {
    let empty_map = Value::Map(vec![]);
    let mut buf = vec![];
    into_writer(&empty_map, &mut buf).unwrap();
    let result: Result<NonEmptyMapCbor, _> = from_reader(buf.as_slice());
    assert!(result.is_err());
}

// --- non-empty constraint tests (array) ---

#[derive(Clone, Debug, PartialEq, Eq, StructToArray, Serialize, Deserialize)]
#[cbor(non_empty = "true")]
pub struct NonEmptyArray {
    #[cbor(value = "Text")]
    pub field_a: Option<String>,
    #[cbor(value = "Text")]
    pub field_b: Option<String>,
}

#[test]
fn non_empty_array_serialize_with_field_present() {
    let orig = NonEmptyArrayCbor {
        field_a: Some("hello".to_string()),
        field_b: None,
    };
    let mut buf = vec![];
    into_writer(&orig, &mut buf).unwrap();
    let decoded: NonEmptyArrayCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(orig, decoded);
}

#[test]
fn non_empty_array_serialize_all_none_fails() {
    let empty = NonEmptyArrayCbor {
        field_a: None,
        field_b: None,
    };
    let mut buf = vec![];
    let result = into_writer(&empty, &mut buf);
    assert!(result.is_err());
}

// --- Malformed input errors ---

#[test]
fn struct_to_array_malformed_input() {
    // A CBOR map should fail to decode as an array-based struct
    let map_value = Value::Map(vec![(
        Value::Integer(0.into()),
        Value::Text("nope".to_string()),
    )]);
    let mut buf = vec![];
    into_writer(&map_value, &mut buf).unwrap();
    let result: Result<SimpleArrayCbor, _> = from_reader(buf.as_slice());
    assert!(result.is_err());
}

#[test]
fn struct_to_map_malformed_input() {
    // A CBOR array should fail to decode as a map-based struct
    let arr_value = Value::Array(vec![Value::Text("nope".to_string())]);
    let mut buf = vec![];
    into_writer(&arr_value, &mut buf).unwrap();
    let result: Result<SimpleMapCbor, _> = from_reader(buf.as_slice());
    assert!(result.is_err());
}
