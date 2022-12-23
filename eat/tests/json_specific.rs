use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use common::TextOrInt;
use eat::arrays::*;
use hex_literal::hex;
use std::path::Path;

use eat::json_specific::*;

mod utils;
use utils::*;

#[test]
fn json_selector_type_test() {
    let options = ["JWT", "CBOR", "BUNDLE", "DIGEST", "OTHER"];
    for o in options {
        let scratch = JsonSelectorType::try_from(o.to_string()).unwrap();
        match &scratch {
            JsonSelectorType::Jwt => assert_eq!("JWT", o),
            JsonSelectorType::Cbor => assert_eq!("CBOR", o),
            JsonSelectorType::Bundle => assert_eq!("BUNDLE", o),
            JsonSelectorType::Digest => assert_eq!("DIGEST", o),
            JsonSelectorType::Other(v) => {
                assert_eq!("OTHER", o);
                assert_eq!("OTHER", v);
            }
        };

        let mut encoded_cbor = vec![];
        let _ = into_writer(&scratch, &mut encoded_cbor);
        println!(
            "Encoded JsonSelectorType: {:?}",
            buffer_to_hex(encoded_cbor.as_slice())
        );

        let decoded: JsonSelectorType = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let mut encoded_cbor_roundtrip = vec![];
        let r = into_writer(&decoded, &mut encoded_cbor_roundtrip);
        assert!(r.is_ok());
        assert_eq!(encoded_cbor, encoded_cbor_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");
    }
}

#[test]
fn json_selector_value_test() {
    let jwt_str = get_file_as_byte_vec(Path::new("tests/examples/a1_jwe.jwt"));
    let jwt = std::str::from_utf8(&jwt_str).unwrap();
    //todo replace with signed EAT example
    let eatbytes = hex!("b0016941636d6520496e632e026772722d74726170036941636d6520496e632e04c10005c10006c1000746ffffffffffff0a4800000000000000000b5101deadbeefdeadbeefdeadbeefdeadbeef0c6941636d6520496e632e0d46ffffffffffff0e030ff5100111a201fb4028ae147ae147ae02fb404c63d70a3d70a413183c");
    let eatbase64 = base64::encode(eatbytes);
    let digest = &eatbytes.as_slice()[0..32];

    //todo replace with proper detached EAT bundle (this just uses bytes from above for both main taken
    //and detached claims and there is no submodule with detached digest
    let sel4deb = JsonSelectorForDebValue::CborTokenInsideJsonToken(eatbase64.to_string());
    let sel = SelectorForDeb {
        token_type: JsonSelectorType::Cbor,
        nested_token: sel4deb,
    };
    let nc = NestedToken(Box::new(sel));
    let deb = DetachedEatBundle {
        main_token: nc,
        detached_claims_set: vec![WrappedClaimsSet(eatbase64.to_string())],
    };

    //todo replace with actual detached submodule digest
    let dsd = DetachedSubmoduleDigest {
        hash_algorithm: TextOrInt::Int(1),
        digest: digest.to_vec(),
    };

    for i in 0..4 {
        let scratch = match i {
            0 => JsonSelectorValue::JwtMessage(jwt.to_string()),
            1 => JsonSelectorValue::CborTokenInsideJsonToken(eatbase64.clone()),
            2 => JsonSelectorValue::DetachedEatBundle(deb.clone()),
            3 => JsonSelectorValue::DetachedSubmoduleDigest(dsd.clone()),
            _ => panic!(),
        };

        let encoded_json = serde_json::to_string(&scratch).unwrap();
        let decoded_json: JsonSelectorValue = serde_json::from_str(encoded_json.as_str()).unwrap();
        assert_eq!(scratch, decoded_json);
    }
}

#[test]
fn json_selector_for_deb_value_test() {
    let jwt_str = get_file_as_byte_vec(Path::new("tests/examples/a1_jwe.jwt"));
    let jwt = std::str::from_utf8(&jwt_str).unwrap();

    //todo replace with signed EAT example
    let eatbytes = hex!("b0016941636d6520496e632e026772722d74726170036941636d6520496e632e04c10005c10006c1000746ffffffffffff0a4800000000000000000b5101deadbeefdeadbeefdeadbeefdeadbeef0c6941636d6520496e632e0d46ffffffffffff0e030ff5100111a201fb4028ae147ae147ae02fb404c63d70a3d70a413183c");
    let eatbase64 = base64::encode(eatbytes);
    let digest = &eatbytes.as_slice()[0..32];

    //todo replace with actual detached submodule digest
    let dsd = DetachedSubmoduleDigest {
        hash_algorithm: TextOrInt::Int(1),
        digest: digest.to_vec(),
    };

    for i in 0..3 {
        let scratch = match i {
            0 => JsonSelectorValue::JwtMessage(jwt.to_string()),
            1 => JsonSelectorValue::CborTokenInsideJsonToken(eatbase64.clone()),
            2 => JsonSelectorValue::DetachedSubmoduleDigest(dsd.clone()),
            _ => panic!(),
        };

        let encoded_json = serde_json::to_string(&scratch).unwrap();
        let decoded_json: JsonSelectorValue = serde_json::from_str(encoded_json.as_str()).unwrap();
        assert_eq!(scratch, decoded_json);
    }
}
