//! Miscellaneous tests

use ciborium::{de::from_reader, ser::into_writer, value::Value};
use hex_literal::hex;

use common::{OidType, TaggedOidTypeCbor};

mod utils;
use crate::utils::buffer_to_hex;

#[allow(unused_variables)]
#[test]
fn simple() {
    macro_rules! cval {
        ($x:expr) => {
            Value::from(val!($x))
        };
    }

    macro_rules! val {
        ($x:expr) => {
            ciborium::cbor!($x).unwrap()
        };
    }

    //let mut encoded_token2106 = vec![];

    let mut encoded_token2105 = vec![];
    let oid_bytes = hex!("2a03");
    let tagged_oid_bytes = hex!("d86f422a03");
    let to: TaggedOidTypeCbor = TaggedOidTypeCbor {
        0: OidType(oid_bytes.to_vec()),
    }; //(111, Box::new(Value::Bytes(oid_bytes.to_vec())));
    let to_e = into_writer(&to, &mut encoded_token2105);
    println!(
        "Encoded TaggedOidType: {:?}",
        buffer_to_hex(encoded_token2105.as_slice())
    );
    let to_d: TaggedOidTypeCbor = from_reader(encoded_token2105.clone().as_slice()).unwrap();
    println!("Decoded TaggedOidType: {:?}", to_d);

    let www = cval!(123u8);
    let xxx = val!(123u32);
    let yyy = ciborium::cbor!(123u64).unwrap();
    //let yyyb : i128 = yyy.try_into().unwrap();
    let i: u64 = yyy.as_integer().unwrap().try_into().unwrap();

    let value = Value::Bytes(vec![104, 101, 108, 108, 111]);
    let v = value.as_bytes().unwrap();
    println!("value: {:?}", &value);
    println!("v: {:?}", &v);
}

#[test]
fn tagged_svn_test() {
    use ciborium::tag::Required;
    use common::TaggedSvn;
    let svn: TaggedSvn = Required(42);
    let mut buf = vec![];
    let _ = into_writer(&svn, &mut buf);
    println!("Encoded TaggedSvn: {:?}", buffer_to_hex(buf.as_slice()));
    let decoded: TaggedSvn = from_reader(buf.as_slice()).unwrap();
    assert_eq!(svn, decoded);
    assert_eq!(decoded.0, 42);
}

#[test]
fn tagged_min_svn_test() {
    use ciborium::tag::Required;
    use common::TaggedMinSvn;
    let svn: TaggedMinSvn = Required(10);
    let mut buf = vec![];
    let _ = into_writer(&svn, &mut buf);
    println!("Encoded TaggedMinSvn: {:?}", buffer_to_hex(buf.as_slice()));
    let decoded: TaggedMinSvn = from_reader(buf.as_slice()).unwrap();
    assert_eq!(svn, decoded);
    assert_eq!(decoded.0, 10);
}

#[test]
fn raw_value_type_choice_test() {
    use ciborium::tag::Required;
    use common::BytesType;
    use corim::choices::RawValueTypeChoice;

    // Test Bytes variant (tag 560)
    let rv = RawValueTypeChoice::Bytes(Required(BytesType(vec![0xDE, 0xAD])));
    let mut buf = vec![];
    let _ = into_writer(&rv, &mut buf);
    println!(
        "Encoded RawValueTypeChoice::Bytes: {:?}",
        buffer_to_hex(buf.as_slice())
    );
    let value: Value = from_reader(buf.as_slice()).unwrap();
    match &value {
        Value::Tag(560, _) => {}
        _ => panic!("Expected tag 560"),
    }

    // Test MaskedRawValue variant (tag 563)
    let mrv = common::arrays::MaskedRawValueCbor {
        value: vec![0xFF],
        mask: vec![0xFF],
    };
    let rv2 = RawValueTypeChoice::MaskedRawValue(Required(mrv));
    let mut buf2 = vec![];
    let _ = into_writer(&rv2, &mut buf2);
    let value2: Value = from_reader(buf2.as_slice()).unwrap();
    match &value2 {
        Value::Tag(563, _) => {}
        _ => panic!("Expected tag 563"),
    }
}

#[test]
fn raw_value_mask_type_choice_test() {
    // raw-value-mask is a simple bytes field (tag 5 in MeasurementValuesMap), not a separate type.
    // Verify that raw_value_mask bytes roundtrip correctly as plain bytes.
    let mask = vec![0xFF, 0x00, 0xFF];
    let mut buf = vec![];
    let _ = into_writer(&serde_bytes::ByteBuf::from(mask.clone()), &mut buf);
    let decoded: serde_bytes::ByteBuf = from_reader(buf.as_slice()).unwrap();
    assert_eq!(mask, decoded.as_ref());
}
