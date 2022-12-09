extern crate alloc;
use alloc::string::{String, ToString};
use ciborium::value::{Integer, Value};
use common::tuple::TupleCbor;
use common::tuple_map::*;

#[allow(dead_code)]
#[allow(missing_docs)]
pub fn buffer_to_hex(buffer: &[u8]) -> String {
    let hex = subtle_encoding::hex::encode_upper(buffer);
    let r = std::str::from_utf8(hex.as_slice());
    if let Ok(s) = r {
        s.to_string()
    } else {
        "".to_string()
    }
}
#[test]
fn tuple_test() {
    use ciborium::de::from_reader;
    use ciborium::ser::into_writer;
    let t = TupleCbor {
        key: Value::Integer(Integer::from(6)),
        value: Value::Text("Blah".to_string()),
    };
    let mut encoded_token2 = vec![];
    let _ = into_writer(&t, &mut encoded_token2);
    println!(
        "Encoded TupleCbor: {:?}",
        buffer_to_hex(encoded_token2.as_slice())
    );

    let egl_d: TupleCbor = from_reader(encoded_token2.clone().as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(encoded_token, encoded_token2);
}

#[test]
fn tuple_map_test() {
    use ciborium::de::from_reader;
    use ciborium::ser::into_writer;
    let t1 = TupleCbor {
        key: Value::Text("Key1".to_string()),
        value: Value::Text("Value1".to_string()),
    };
    let t2 = TupleCbor {
        key: Value::Text("Key2".to_string()),
        value: Value::Text("Value2".to_string()),
    };
    let tm = TupleMapCbor {
        tuples: vec![t1, t2],
    };
    let mut encoded_token2 = vec![];
    let _ = into_writer(&tm, &mut encoded_token2);
    println!(
        "Encoded TupleCbor: {:?}",
        buffer_to_hex(encoded_token2.as_slice())
    );

    let egl_d: TupleMapCbor = from_reader(encoded_token2.as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(encoded_token, encoded_token2);
}
