// Tests for the discovery module based on examples from
// draft-ietf-rats-coserv-05 Section 6.1.1.3.

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ciborium::value::Value;

use std::collections::BTreeMap;

use common::{TextOrInt, Tuple, TupleCbor};
use cose::maps::{CoseKey, CoseKeyCbor, CoseKeySet};
use coserv::discovery::*;
use hex_literal::hex;

// -- Helper: build the spec CBOR example as a Value tree --

/// Builds the CBOR EDN example from Section 6.1.1.3 as a ciborium Value.
///
/// {
///   1: "1.2.3-beta",
///   2: [{
///     1: "application/coserv+cose; profile=\"tag:vendor.com,2025:cc_platform#1.0.0\"",
///     2: ["source", "collected"]
///   }],
///   3: {
///     "CoSERVRequestResponse": "/endorsement-distribution/v1/coserv/{query}"
///   },
///   4: [{
///     1: 2, 2: h'ABCDEF1234', 3: -7,
///     -1: 1, -2: h'1A2B3C4D', -3: h'5E6F7A8B'
///   }]
/// }
fn spec_cbor_value() -> Value {
    let media_type = "application/coserv+cose; profile=\"tag:vendor.com,2025:cc_platform#1.0.0\"";
    let endpoint_path = "/endorsement-distribution/v1/coserv/{query}";

    let capability = Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text(media_type.to_string()),
        ),
        (
            Value::Integer(2.into()),
            Value::Array(vec![
                Value::Text("source".to_string()),
                Value::Text("collected".to_string()),
            ]),
        ),
    ]);

    let api_endpoints = Value::Map(vec![(
        Value::Text("CoSERVRequestResponse".to_string()),
        Value::Text(endpoint_path.to_string()),
    )]);

    // COSE_Key with integer labels (including negative labels for EC params)
    let cose_key = Value::Map(vec![
        (
            Value::Integer((-3).into()),
            Value::Bytes(hex!("5E6F7A8B").to_vec()),
        ),
        (
            Value::Integer((-2).into()),
            Value::Bytes(hex!("1A2B3C4D").to_vec()),
        ),
        (Value::Integer((-1).into()), Value::Integer(1.into())),
        (Value::Integer(1.into()), Value::Integer(2.into())),
        (
            Value::Integer(2.into()),
            Value::Bytes(hex!("ABCDEF1234").to_vec()),
        ),
        (Value::Integer(3.into()), Value::Integer((-7).into())),
    ]);

    Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text("1.2.3-beta".to_string()),
        ),
        (Value::Integer(2.into()), Value::Array(vec![capability])),
        (Value::Integer(3.into()), api_endpoints),
        (Value::Integer(4.into()), Value::Array(vec![cose_key])),
    ])
}

const MEDIA_TYPE: &str =
    "application/coserv+cose; profile=\"tag:vendor.com,2025:cc_platform#1.0.0\"";
const ENDPOINT_PATH: &str = "/endorsement-distribution/v1/coserv/{query}";

/// Build the expected CoservWellKnownInfoMap (JSON-friendly form).
fn spec_json_form() -> CoservWellKnownInfoMap {
    let mut api_endpoints = BTreeMap::new();
    api_endpoints.insert(
        "CoSERVRequestResponse".to_string(),
        ENDPOINT_PATH.to_string(),
    );

    CoservWellKnownInfoMap {
        version: "1.2.3-beta".to_string(),
        capabilities: vec![CapabilityMap {
            media_type: MEDIA_TYPE.to_string(),
            artifact_support: vec![ArtifactSupportType::Source, ArtifactSupportType::Collected],
        }],
        api_endpoints,
        result_verification_key: Some(CoseKeySet(vec![CoseKey {
            kty: Some(TextOrInt::Int(2)),
            kid: Some(hex!("ABCDEF1234").to_vec()),
            alg: Some(TextOrInt::Int(-7)),
            key_ops: None,
            iv: None,
            other: Some(vec![
                Tuple {
                    key: Value::Integer((-1).into()),
                    value: Value::Integer(1.into()),
                },
                Tuple {
                    key: Value::Integer((-2).into()),
                    value: Value::Bytes(hex!("1A2B3C4D").to_vec()),
                },
                Tuple {
                    key: Value::Integer((-3).into()),
                    value: Value::Bytes(hex!("5E6F7A8B").to_vec()),
                },
            ]),
        }])),
    }
}

/// Build the expected CoservWellKnownInfoMapCbor form.
fn spec_cbor_form() -> CoservWellKnownInfoMapCbor {
    let mut api_endpoints = BTreeMap::new();
    api_endpoints.insert(
        "CoSERVRequestResponse".to_string(),
        ENDPOINT_PATH.to_string(),
    );

    CoservWellKnownInfoMapCbor {
        version: "1.2.3-beta".to_string(),
        capabilities: vec![CapabilityMapCbor {
            media_type: MEDIA_TYPE.to_string(),
            artifact_support: vec![ArtifactSupportType::Source, ArtifactSupportType::Collected],
        }],
        api_endpoints,
        result_verification_key: Some(vec![CoseKeyCbor {
            kty: Some(TextOrInt::Int(2)),
            kid: Some(hex!("ABCDEF1234").to_vec()),
            alg: Some(TextOrInt::Int(-7)),
            key_ops: None,
            iv: None,
            other: Some(vec![
                TupleCbor {
                    key: Value::Integer((-1).into()),
                    value: Value::Integer(1.into()),
                },
                TupleCbor {
                    key: Value::Integer((-2).into()),
                    value: Value::Bytes(hex!("1A2B3C4D").to_vec()),
                },
                TupleCbor {
                    key: Value::Integer((-3).into()),
                    value: Value::Bytes(hex!("5E6F7A8B").to_vec()),
                },
            ]),
        }]),
    }
}

// -- ArtifactSupportType tests --

#[test]
fn artifact_support_type_from_value() {
    let source = ArtifactSupportType::try_from(Value::Text("source".to_string())).unwrap();
    assert_eq!(source, ArtifactSupportType::Source);

    let collected = ArtifactSupportType::try_from(Value::Text("collected".to_string())).unwrap();
    assert_eq!(collected, ArtifactSupportType::Collected);
}

#[test]
fn artifact_support_type_invalid() {
    let err = ArtifactSupportType::try_from(Value::Text("unknown".to_string()));
    assert!(err.is_err());

    let err = ArtifactSupportType::try_from(Value::Integer(0.into()));
    assert!(err.is_err());
}

// -- CapabilityMap tests --

#[test]
fn capability_cbor_roundtrip() {
    let cap = CapabilityMapCbor {
        media_type: MEDIA_TYPE.to_string(),
        artifact_support: vec![ArtifactSupportType::Source, ArtifactSupportType::Collected],
    };

    let mut buf = vec![];
    into_writer(&cap, &mut buf).unwrap();
    let decoded: CapabilityMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(cap, decoded);
}

#[test]
fn capability_json_roundtrip() {
    let cap = CapabilityMap {
        media_type: MEDIA_TYPE.to_string(),
        artifact_support: vec![ArtifactSupportType::Source, ArtifactSupportType::Collected],
    };

    let json = serde_json::to_string(&cap).unwrap();
    assert!(json.contains("\"media-type\""));
    assert!(json.contains("\"artifact-support\""));
    let decoded: CapabilityMap = serde_json::from_str(&json).unwrap();
    assert_eq!(cap, decoded);
}

#[test]
fn capability_cbor_to_json_roundtrip() {
    let cbor_form = CapabilityMapCbor {
        media_type: MEDIA_TYPE.to_string(),
        artifact_support: vec![ArtifactSupportType::Source],
    };

    let json_form: CapabilityMap = CapabilityMap::try_from(&cbor_form).unwrap();
    assert_eq!(json_form.media_type, MEDIA_TYPE);

    let back: CapabilityMapCbor = CapabilityMapCbor::try_from(&json_form).unwrap();
    assert_eq!(cbor_form, back);
}

// -- Validation tests --

#[test]
fn capability_validate_empty_artifact_support() {
    let cap = CapabilityMap {
        media_type: "application/example".to_string(),
        artifact_support: vec![],
    };
    assert!(cap.validate().is_err());
}

#[test]
fn capability_validate_ok() {
    let cap = CapabilityMap {
        media_type: "application/example".to_string(),
        artifact_support: vec![ArtifactSupportType::Source],
    };
    assert!(cap.validate().is_ok());
}

#[test]
fn well_known_validate_empty_capabilities() {
    let wki = CoservWellKnownInfoMap {
        version: "1.0.0".to_string(),
        capabilities: vec![],
        api_endpoints: BTreeMap::from([("k".to_string(), "v".to_string())]),
        result_verification_key: None,
    };
    assert!(wki.validate().is_err());
}

#[test]
fn well_known_validate_empty_api_endpoints() {
    let wki = CoservWellKnownInfoMap {
        version: "1.0.0".to_string(),
        capabilities: vec![CapabilityMap {
            media_type: "application/example".to_string(),
            artifact_support: vec![ArtifactSupportType::Collected],
        }],
        api_endpoints: BTreeMap::new(),
        result_verification_key: None,
    };
    assert!(wki.validate().is_err());
}

#[test]
fn well_known_validate_ok() {
    let wki = spec_json_form();
    assert!(wki.validate().is_ok());
}

// -- CoservWellKnownInfoMapCbor: decode from spec CBOR example --

#[test]
fn well_known_cbor_decode_spec_example() {
    // Encode the spec EDN as CBOR bytes
    let value = spec_cbor_value();
    let mut cbor_bytes = vec![];
    into_writer(&value, &mut cbor_bytes).unwrap();

    // Decode as our type
    let decoded: CoservWellKnownInfoMapCbor = from_reader(cbor_bytes.as_slice()).unwrap();
    assert_eq!(decoded.version, "1.2.3-beta");
    assert_eq!(decoded.capabilities.len(), 1);
    assert_eq!(decoded.capabilities[0].media_type, MEDIA_TYPE);
    assert_eq!(
        decoded.capabilities[0].artifact_support,
        vec![ArtifactSupportType::Source, ArtifactSupportType::Collected]
    );
    assert_eq!(
        decoded.api_endpoints.get("CoSERVRequestResponse").unwrap(),
        ENDPOINT_PATH
    );
    assert!(decoded.result_verification_key.is_some());
    let keys = decoded.result_verification_key.as_ref().unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].kty, Some(TextOrInt::Int(2)));
    assert_eq!(keys[0].alg, Some(TextOrInt::Int(-7)));
    assert_eq!(keys[0].kid, Some(hex!("ABCDEF1234").to_vec()));
}

#[test]
fn well_known_cbor_roundtrip() {
    let original = spec_cbor_form();

    let mut buf = vec![];
    into_writer(&original, &mut buf).unwrap();
    let decoded: CoservWellKnownInfoMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(original, decoded);
}

// -- CoservWellKnownInfoMap: JSON roundtrip --

#[test]
fn well_known_json_roundtrip() {
    // Use a form without result-verification-key for JSON roundtrip, since the
    // spec defines JWK_Set (not COSE_KeySet) for JSON encoding and CoseKey's
    // Tuple byte values don't roundtrip losslessly through JSON.
    let mut original = spec_json_form();
    original.result_verification_key = None;

    let json = serde_json::to_string_pretty(&original).unwrap();
    println!("JSON:\n{json}");

    // Verify JSON key names match the spec
    assert!(json.contains("\"version\""));
    assert!(json.contains("\"capabilities\""));
    assert!(json.contains("\"api-endpoints\""));
    assert!(json.contains("\"media-type\""));
    assert!(json.contains("\"artifact-support\""));
    assert!(!json.contains("\"result-verification-key\""));

    let decoded: CoservWellKnownInfoMap = serde_json::from_str(&json).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn well_known_json_key_names() {
    // Verify the JSON key names are correct even with all fields present
    let original = spec_json_form();
    let json = serde_json::to_string_pretty(&original).unwrap();

    assert!(json.contains("\"version\""));
    assert!(json.contains("\"capabilities\""));
    assert!(json.contains("\"api-endpoints\""));
    assert!(json.contains("\"result-verification-key\""));
    assert!(json.contains("\"media-type\""));
    assert!(json.contains("\"artifact-support\""));
}

// -- CBOR ↔ JSON conversion --

#[test]
fn well_known_cbor_to_json_conversion() {
    let cbor_form = spec_cbor_form();
    let json_form: CoservWellKnownInfoMap = CoservWellKnownInfoMap::try_from(&cbor_form).unwrap();

    assert_eq!(json_form.version, "1.2.3-beta");
    assert_eq!(json_form.capabilities.len(), 1);
    assert_eq!(json_form.capabilities[0].media_type, MEDIA_TYPE);
    assert!(json_form.result_verification_key.is_some());
    let ks = json_form.result_verification_key.as_ref().unwrap();
    assert_eq!(ks.0.len(), 1);
    assert_eq!(ks.0[0].kty, Some(TextOrInt::Int(2)));
}

#[test]
fn well_known_json_to_cbor_conversion() {
    let json_form = spec_json_form();
    let cbor_form: CoservWellKnownInfoMapCbor =
        CoservWellKnownInfoMapCbor::try_from(&json_form).unwrap();

    assert_eq!(cbor_form.version, "1.2.3-beta");
    assert_eq!(cbor_form.capabilities.len(), 1);
    assert!(cbor_form.result_verification_key.is_some());
    let keys = cbor_form.result_verification_key.as_ref().unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].kty, Some(TextOrInt::Int(2)));
}

#[test]
fn well_known_full_roundtrip_cbor_json_cbor() {
    // Use a form without result-verification-key for the full CBOR→JSON→CBOR
    // roundtrip, since CoseKey Tuple byte values don't survive JSON encoding.
    let value = Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text("1.2.3-beta".to_string()),
        ),
        (
            Value::Integer(2.into()),
            Value::Array(vec![Value::Map(vec![
                (
                    Value::Integer(1.into()),
                    Value::Text(MEDIA_TYPE.to_string()),
                ),
                (
                    Value::Integer(2.into()),
                    Value::Array(vec![
                        Value::Text("source".to_string()),
                        Value::Text("collected".to_string()),
                    ]),
                ),
            ])]),
        ),
        (
            Value::Integer(3.into()),
            Value::Map(vec![(
                Value::Text("CoSERVRequestResponse".to_string()),
                Value::Text(ENDPOINT_PATH.to_string()),
            )]),
        ),
    ]);
    let mut cbor_bytes = vec![];
    into_writer(&value, &mut cbor_bytes).unwrap();

    // Decode as CBOR type
    let cbor_form: CoservWellKnownInfoMapCbor = from_reader(cbor_bytes.as_slice()).unwrap();

    // Convert to JSON type
    let json_form: CoservWellKnownInfoMap = CoservWellKnownInfoMap::try_from(&cbor_form).unwrap();

    // Roundtrip through JSON string
    let json = serde_json::to_string(&json_form).unwrap();
    let dec_json: CoservWellKnownInfoMap = serde_json::from_str(&json).unwrap();

    // Convert back to CBOR type
    let roundtrip: CoservWellKnownInfoMapCbor =
        CoservWellKnownInfoMapCbor::try_from(&dec_json).unwrap();

    // Re-encode and compare
    let mut roundtrip_bytes = vec![];
    into_writer(&roundtrip, &mut roundtrip_bytes).unwrap();
    assert_eq!(cbor_bytes, roundtrip_bytes);
}

// -- Without result-verification-key (optional field) --

#[test]
fn well_known_cbor_without_verification_key() {
    let value = Value::Map(vec![
        (Value::Integer(1.into()), Value::Text("0.1.0".to_string())),
        (
            Value::Integer(2.into()),
            Value::Array(vec![Value::Map(vec![
                (
                    Value::Integer(1.into()),
                    Value::Text("application/coserv+cbor".to_string()),
                ),
                (
                    Value::Integer(2.into()),
                    Value::Array(vec![Value::Text("collected".to_string())]),
                ),
            ])]),
        ),
        (
            Value::Integer(3.into()),
            Value::Map(vec![(
                Value::Text("CoSERVRequestResponse".to_string()),
                Value::Text("/api/v1/{query}".to_string()),
            )]),
        ),
    ]);

    let mut buf = vec![];
    into_writer(&value, &mut buf).unwrap();
    let decoded: CoservWellKnownInfoMapCbor = from_reader(buf.as_slice()).unwrap();

    assert_eq!(decoded.version, "0.1.0");
    assert!(decoded.result_verification_key.is_none());
    assert_eq!(decoded.capabilities[0].artifact_support.len(), 1);

    // roundtrip
    let mut roundtrip_buf = vec![];
    into_writer(&decoded, &mut roundtrip_buf).unwrap();
    assert_eq!(buf, roundtrip_buf);
}

#[test]
fn well_known_json_without_verification_key() {
    let mut api_endpoints = BTreeMap::new();
    api_endpoints.insert(
        "CoSERVRequestResponse".to_string(),
        "/api/{query}".to_string(),
    );

    let original = CoservWellKnownInfoMap {
        version: "0.1.0".to_string(),
        capabilities: vec![CapabilityMap {
            media_type: "application/coserv+cbor".to_string(),
            artifact_support: vec![ArtifactSupportType::Collected],
        }],
        api_endpoints,
        result_verification_key: None,
    };

    let json = serde_json::to_string(&original).unwrap();
    // result-verification-key should be omitted, not null
    assert!(!json.contains("result-verification-key"));
    let decoded: CoservWellKnownInfoMap = serde_json::from_str(&json).unwrap();
    assert_eq!(original, decoded);
}
