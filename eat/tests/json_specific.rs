use std::path::Path;

use base64::{Engine, engine::general_purpose::STANDARD};
use ciborium::{de::from_reader, ser::into_writer};
use hex_literal::hex;

use common::TextOrInt;
use eat::{
    arrays::{DetachedEatBundle, DetachedSubmoduleDigest, NestedToken, WrappedClaimsSet},
    json_specific::{
        JsonSelector, JsonSelectorForDebValue, JsonSelectorType, JsonSelectorValue, SelectorForDeb,
    },
};

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

    let eatbytes = hex!(
        "b0016941636d6520496e632e026772722d74726170036941636d6520496e632e04c10005c10006c1000746ffffffffffff0a4800000000000000000b5101deadbeefdeadbeefdeadbeefdeadbeef0c6941636d6520496e632e0d46ffffffffffff0e030ff5100111a201fb4028ae147ae147ae02fb404c63d70a3d70a413183c"
    );
    let eatbase64 = STANDARD.encode(eatbytes);
    let digest = &eatbytes.as_slice()[0..32];

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

    let dsd = DetachedSubmoduleDigest {
        hash_algorithm: TextOrInt::Int(1),
        digest: digest.to_vec(),
    };

    // Test each variant through JsonSelector (which uses token_type to drive deserialization)
    let selectors = vec![
        JsonSelector {
            token_type: JsonSelectorType::Jwt,
            nested_token: JsonSelectorValue::JwtMessage(jwt.to_string()),
        },
        JsonSelector {
            token_type: JsonSelectorType::Cbor,
            nested_token: JsonSelectorValue::CborTokenInsideJsonToken(eatbase64.clone()),
        },
        JsonSelector {
            token_type: JsonSelectorType::Bundle,
            nested_token: JsonSelectorValue::DetachedEatBundle(deb.clone()),
        },
        JsonSelector {
            token_type: JsonSelectorType::Digest,
            nested_token: JsonSelectorValue::DetachedSubmoduleDigest(dsd.clone()),
        },
    ];

    for selector in &selectors {
        let encoded_json = serde_json::to_string(selector).unwrap();
        let decoded: JsonSelector = serde_json::from_str(&encoded_json).unwrap();
        assert_eq!(selector.token_type, decoded.token_type);
        assert_eq!(selector.nested_token, decoded.nested_token);
    }
}

#[test]
fn json_selector_for_deb_value_test() {
    let jwt_str = get_file_as_byte_vec(Path::new("tests/examples/a1_jwe.jwt"));
    let jwt = std::str::from_utf8(&jwt_str).unwrap();

    let eatbytes = hex!(
        "b0016941636d6520496e632e026772722d74726170036941636d6520496e632e04c10005c10006c1000746ffffffffffff0a4800000000000000000b5101deadbeefdeadbeefdeadbeefdeadbeef0c6941636d6520496e632e0d46ffffffffffff0e030ff5100111a201fb4028ae147ae147ae02fb404c63d70a3d70a413183c"
    );
    let eatbase64 = STANDARD.encode(eatbytes);
    let digest = &eatbytes.as_slice()[0..32];

    let dsd = DetachedSubmoduleDigest {
        hash_algorithm: TextOrInt::Int(1),
        digest: digest.to_vec(),
    };

    // Test each variant through SelectorForDeb
    let selectors = vec![
        SelectorForDeb {
            token_type: JsonSelectorType::Jwt,
            nested_token: JsonSelectorForDebValue::JwtMessage(jwt.to_string()),
        },
        SelectorForDeb {
            token_type: JsonSelectorType::Cbor,
            nested_token: JsonSelectorForDebValue::CborTokenInsideJsonToken(eatbase64.clone()),
        },
        SelectorForDeb {
            token_type: JsonSelectorType::Digest,
            nested_token: JsonSelectorForDebValue::DetachedSubmoduleDigest(dsd.clone()),
        },
    ];

    for selector in &selectors {
        let encoded_json = serde_json::to_string(selector).unwrap();
        let decoded: SelectorForDeb = serde_json::from_str(&encoded_json).unwrap();
        assert_eq!(selector.token_type, decoded.token_type);
        assert_eq!(selector.nested_token, decoded.nested_token);
    }
}

/// BUNDLE type is rejected inside a SelectorForDeb
#[test]
fn selector_for_deb_rejects_bundle_type() {
    let json = r#"["BUNDLE", {}]"#;
    let result: Result<SelectorForDeb, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not permitted"));
}
