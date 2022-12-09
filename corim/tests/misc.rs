//! Miscellaneous tests
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use common::{OidType, TaggedOidTypeCbor};
use hex_literal::hex;
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
        0: OidType::Oid(oid_bytes.to_vec()),
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
    //todo
}

#[test]
fn tagged_min_svn_test() {
    //todo
}

#[test]
fn raw_value_type_choice_test() {
    //todo
}

#[test]
fn raw_value_mask_type_choice_test() {
    //todo
}
