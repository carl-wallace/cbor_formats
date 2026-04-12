use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use hex_literal::hex;

use ar4si::choices::*;
use ar4si::maps::*;

// ===========================================================================
// TrustworthinessTier tests
// ===========================================================================

#[test]
fn trustworthiness_tier_values() {
    assert_eq!(TrustworthinessTier::None as i8, 0);
    assert_eq!(TrustworthinessTier::Affirming as i8, 2);
    assert_eq!(TrustworthinessTier::Warning as i8, 32);
    assert_eq!(TrustworthinessTier::Contraindicated as i8, 96);
}

#[test]
fn trustworthiness_tier_cbor_roundtrip() {
    for tier in [
        TrustworthinessTier::None,
        TrustworthinessTier::Affirming,
        TrustworthinessTier::Warning,
        TrustworthinessTier::Contraindicated,
    ] {
        let mut buf = vec![];
        into_writer(&tier, &mut buf).unwrap();
        let decoded: TrustworthinessTier = from_reader(buf.as_slice()).unwrap();
        assert_eq!(decoded, tier);
    }
}

// ===========================================================================
// VerifierId tests
// ===========================================================================

/// CBOR from draft-ietf-rats-ear-03 Figure 5 verifier-id fragment:
/// `{ 0: "https://veraison-project.org", 1: "vts 0.0.1" }`
#[test]
fn verifier_id_decode_ear_draft_fig5() {
    let bytes = hex!(
        "a200781c68747470733a2f2f7665726169736f6e2d70726f6a6563742e6f"
        "7267016976747320302e302e31"
    );
    let decoded: VerifierIdCbor = from_reader(bytes.as_slice()).unwrap();
    let vid: VerifierId = (&decoded).try_into().unwrap();
    assert_eq!(vid.developer, "https://veraison-project.org");
    assert_eq!(vid.build, "vts 0.0.1");
}

#[test]
fn verifier_id_roundtrip() {
    let vid = VerifierId {
        developer: "Acme Inc.".to_string(),
        build: "rrtrap-v1.0.0".to_string(),
    };
    let cbor: VerifierIdCbor = (&vid).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: VerifierIdCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: VerifierId = (&decoded).try_into().unwrap();
    assert_eq!(vid, roundtripped);
}

// ===========================================================================
// TrustworthinessVector tests
// ===========================================================================

/// CBOR from draft-ietf-rats-ear-03 Figure 5 trustworthiness-vector fragment:
/// `{ 0: 2, 2: 96, 4: 2 }`
#[test]
fn trustworthiness_vector_decode_ear_draft_fig5() {
    let bytes = hex!("a300020218600402");
    let decoded: TrustworthinessVectorCbor = from_reader(bytes.as_slice()).unwrap();
    let tv: TrustworthinessVector = (&decoded).try_into().unwrap();
    assert_eq!(tv.instance_identity, Some(2));
    assert_eq!(tv.configuration, None);
    assert_eq!(tv.executables, Some(96));
    assert_eq!(tv.file_system, None);
    assert_eq!(tv.hardware, Some(2));
    assert_eq!(tv.runtime_opaque, None);
    assert_eq!(tv.storage_opaque, None);
    assert_eq!(tv.sourced_data, None);
}

#[test]
fn trustworthiness_vector_roundtrip_all_fields() {
    let tv = TrustworthinessVector {
        instance_identity: Some(2),
        configuration: Some(-1),
        executables: Some(32),
        file_system: Some(96),
        hardware: Some(0),
        runtime_opaque: Some(2),
        storage_opaque: Some(-128),
        sourced_data: Some(127),
    };
    let cbor: TrustworthinessVectorCbor = (&tv).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: TrustworthinessVectorCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: TrustworthinessVector = (&decoded).try_into().unwrap();
    assert_eq!(tv, roundtripped);
}

#[test]
fn trustworthiness_vector_roundtrip_partial() {
    let tv = TrustworthinessVector {
        instance_identity: Some(2),
        configuration: None,
        executables: None,
        file_system: None,
        hardware: Some(2),
        runtime_opaque: None,
        storage_opaque: None,
        sourced_data: None,
    };
    let cbor: TrustworthinessVectorCbor = (&tv).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: TrustworthinessVectorCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: TrustworthinessVector = (&decoded).try_into().unwrap();
    assert_eq!(tv, roundtripped);
}
