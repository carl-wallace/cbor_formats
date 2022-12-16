use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;
use common::choices::*;

#[test]
fn version_scheme_cbor_test() {
    let known_vals = vec![
        VersionSchemeKnownCbor::Multipartnumeric,
        VersionSchemeKnownCbor::MultipartnumericSuffix,
        VersionSchemeKnownCbor::AlphaNumeric,
        VersionSchemeKnownCbor::Decimal,
        VersionSchemeKnownCbor::Semver,
    ];

    for kv in &known_vals {
        let scratch = VersionSchemeCbor::Known(kv.clone());
        let mut encoded_cbor = vec![];
        let r = into_writer(&scratch, &mut encoded_cbor);
        assert!(r.is_ok());

        let value: Value = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let json_from_value: VersionSchemeCbor = VersionSchemeCbor::try_from(&value).unwrap();
        assert!(scratch == json_from_value);

        let decoded: VersionSchemeCbor = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let mut encoded_cbor_roundtrip = vec![];
        let r = into_writer(&decoded, &mut encoded_cbor_roundtrip);
        assert!(r.is_ok());
        assert_eq!(encoded_cbor, encoded_cbor_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");

        let json_from_cbor: VersionScheme = scratch.try_into().unwrap();
        let encoded_json = serde_json::to_string(&json_from_cbor).unwrap();
        let decoded_json: VersionScheme = serde_json::from_str(encoded_json.as_str()).unwrap();
        assert_eq!(decoded_json, json_from_cbor);
        let cbor_from_json: VersionSchemeCbor = json_from_cbor.try_into().unwrap();
        assert_eq!(decoded, cbor_from_json);
        match &cbor_from_json {
            VersionSchemeCbor::Known(v) => assert_eq!(kv, v),
            _ => panic!(),
        };
    }

    {
        let scratch = VersionSchemeCbor::Text("Some Version".to_string());
        let scratch_clone = scratch.clone();
        let mut encoded_cbor = vec![];
        into_writer(&scratch, &mut encoded_cbor).unwrap();

        let json_from_cbor: VersionScheme = scratch.try_into().unwrap();
        let _ = serde_json::to_string(&json_from_cbor).unwrap();
        let roundtrip: VersionSchemeCbor = json_from_cbor.try_into().unwrap();
        assert_eq!(roundtrip, scratch_clone);
        match &roundtrip {
            VersionSchemeCbor::Text(v) => assert_eq!("Some Version", v),
            _ => panic!(),
        };
    }

    {
        let scratch = VersionSchemeCbor::IntExtensions(i64::MAX);
        let scratch_clone = scratch.clone();
        let mut encoded_cbor = vec![];
        into_writer(&scratch, &mut encoded_cbor).unwrap();

        let json_from_cbor: VersionScheme = scratch.try_into().unwrap();
        let _ = serde_json::to_string(&json_from_cbor).unwrap();
        let roundtrip: VersionSchemeCbor = json_from_cbor.try_into().unwrap();
        assert_eq!(roundtrip, scratch_clone);
        match &roundtrip {
            VersionSchemeCbor::IntExtensions(v) => assert_eq!(&i64::MAX, v),
            _ => panic!(),
        };
    }

    {
        let scratch = VersionSchemeCbor::IntExtensions(i64::MIN);
        let scratch_clone = scratch.clone();
        let mut encoded_cbor = vec![];
        into_writer(&scratch, &mut encoded_cbor).unwrap();

        let json_from_cbor: VersionScheme = scratch.try_into().unwrap();
        let _ = serde_json::to_string(&json_from_cbor).unwrap();
        let roundtrip: VersionSchemeCbor = json_from_cbor.try_into().unwrap();
        assert_eq!(roundtrip, scratch_clone);
        match &roundtrip {
            VersionSchemeCbor::IntExtensions(v) => assert_eq!(&i64::MIN, v),
            _ => panic!(),
        };
    }
}

#[test]
fn version_scheme_json_test() {
    let known_vals = vec![
        VersionSchemeKnown::Multipartnumeric,
        VersionSchemeKnown::MultipartnumericSuffix,
        VersionSchemeKnown::AlphaNumeric,
        VersionSchemeKnown::Decimal,
        VersionSchemeKnown::Semver,
    ];

    for kv in &known_vals {
        let scratch = VersionScheme::Known(kv.clone());
        let json_encoded = serde_json::to_string(&scratch).unwrap();
        let decoded: VersionScheme = serde_json::from_str(json_encoded.as_str()).unwrap();
        let json_encoded_roundtrip = serde_json::to_string(&decoded).unwrap();
        assert_eq!(json_encoded, json_encoded_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");

        let cbor_from_json: VersionSchemeCbor = scratch.try_into().unwrap();
        let mut encoded_cbor = vec![];
        let r = into_writer(&cbor_from_json, &mut encoded_cbor);
        assert!(r.is_ok());
        let decoded_cbor: VersionSchemeCbor = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        assert_eq!(decoded_cbor, cbor_from_json);
        let json_from_cbor: VersionScheme = cbor_from_json.try_into().unwrap();
        assert_eq!(decoded, json_from_cbor);
        match &json_from_cbor {
            VersionScheme::Known(v) => assert_eq!(kv, v),
            _ => panic!(),
        };
    }

    {
        let scratch = VersionScheme::Text("Some Version".to_string());
        let scratch_clone = scratch.clone();
        let json_encoded = serde_json::to_string(&scratch).unwrap();

        let cbor_from_json: VersionSchemeCbor = scratch.try_into().unwrap();
        let mut encoded_cbor = vec![];
        let r = into_writer(&cbor_from_json, &mut encoded_cbor);
        assert!(r.is_ok());

        let json_from_cbor: VersionScheme = cbor_from_json.try_into().unwrap();
        let roundtrip = serde_json::to_string(&json_from_cbor).unwrap();
        assert_eq!(scratch_clone, json_from_cbor);
        assert_eq!(json_encoded, roundtrip);
        match &json_from_cbor {
            VersionScheme::Text(v) => assert_eq!("Some Version", v),
            _ => panic!(),
        };
    }

    {
        let scratch = VersionScheme::IntExtensions(i64::MAX);
        let scratch_clone = scratch.clone();
        let json_encoded = serde_json::to_string(&scratch).unwrap();

        let cbor_from_json: VersionSchemeCbor = scratch.try_into().unwrap();
        let mut encoded_cbor = vec![];
        let r = into_writer(&cbor_from_json, &mut encoded_cbor);
        assert!(r.is_ok());

        let json_from_cbor: VersionScheme = cbor_from_json.try_into().unwrap();
        let roundtrip = serde_json::to_string(&json_from_cbor).unwrap();
        assert_eq!(scratch_clone, json_from_cbor);
        assert_eq!(json_encoded, roundtrip);
        match &json_from_cbor {
            VersionScheme::IntExtensions(v) => assert_eq!(&i64::MAX, v),
            _ => panic!(),
        };
    }

    {
        let scratch = VersionScheme::IntExtensions(i64::MIN);
        let scratch_clone = scratch.clone();
        let json_encoded = serde_json::to_string(&scratch).unwrap();

        let cbor_from_json: VersionSchemeCbor = scratch.try_into().unwrap();
        let mut encoded_cbor = vec![];
        let r = into_writer(&cbor_from_json, &mut encoded_cbor);
        assert!(r.is_ok());

        let json_from_cbor: VersionScheme = cbor_from_json.try_into().unwrap();
        let roundtrip = serde_json::to_string(&json_from_cbor).unwrap();
        assert_eq!(scratch_clone, json_from_cbor);
        assert_eq!(json_encoded, roundtrip);
        match &json_from_cbor {
            VersionScheme::IntExtensions(v) => assert_eq!(&i64::MIN, v),
            _ => panic!(),
        };
    }
}

#[test]
fn version_scheme_default_cbor_test() {
    let known_vals = vec![
        VersionSchemeKnown::Multipartnumeric,
        VersionSchemeKnown::MultipartnumericSuffix,
        VersionSchemeKnown::AlphaNumeric,
        VersionSchemeKnown::Decimal,
        VersionSchemeKnown::Semver,
    ];

    for kv in &known_vals {
        let scratch = VersionScheme::Known(kv.clone());
        let mut encoded_cbor = vec![];
        let r = into_writer(&scratch, &mut encoded_cbor);
        assert!(r.is_ok());

        let decoded: VersionScheme = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let mut encoded_cbor_roundtrip = vec![];
        let r = into_writer(&decoded, &mut encoded_cbor_roundtrip);
        assert!(r.is_ok());
        assert_eq!(encoded_cbor, encoded_cbor_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");
    }

    {
        let scratch = VersionScheme::Text("Some Version".to_string());
        let mut encoded_cbor = vec![];
        let r = into_writer(&scratch, &mut encoded_cbor);
        assert!(r.is_ok());
        let decoded: VersionScheme = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let mut encoded_cbor_roundtrip = vec![];
        let r = into_writer(&decoded, &mut encoded_cbor_roundtrip);
        assert!(r.is_ok());
        assert_eq!(encoded_cbor, encoded_cbor_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");
    }

    {
        let scratch = VersionScheme::IntExtensions(i64::MAX);
        let mut encoded_cbor = vec![];
        let r = into_writer(&scratch, &mut encoded_cbor);
        assert!(r.is_ok());
        let decoded: VersionScheme = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let mut encoded_cbor_roundtrip = vec![];
        let r = into_writer(&decoded, &mut encoded_cbor_roundtrip);
        assert!(r.is_ok());
        assert_eq!(encoded_cbor, encoded_cbor_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");
    }

    {
        let scratch = VersionScheme::IntExtensions(i64::MIN);
        let mut encoded_cbor = vec![];
        let r = into_writer(&scratch, &mut encoded_cbor);
        assert!(r.is_ok());
        let decoded: VersionScheme = from_reader(encoded_cbor.clone().as_slice()).unwrap();
        let mut encoded_cbor_roundtrip = vec![];
        let r = into_writer(&decoded, &mut encoded_cbor_roundtrip);
        assert!(r.is_ok());
        assert_eq!(encoded_cbor, encoded_cbor_roundtrip);
        assert!(scratch == decoded);
        let _s = format!("{scratch:?}");
    }
}
