use std::collections::BTreeMap;

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use hex_literal::hex;

use ar4si::choices::TrustworthinessTier;
use ar4si::maps::TrustworthinessVector;
use ear::maps::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_ear() -> Ear {
    let mut submods = BTreeMap::new();
    submods.insert(
        "attester".to_string(),
        EarAppraisal {
            profile: None,
            status: TrustworthinessTier::Affirming,
            trustworthiness_vector: None,
            appraisal_policy_ids: None,
            nonce: None,
            other: None,
        },
    );
    Ear {
        profile: "tag:ietf.org,2026:rats/ear#03".to_string(),
        status: None,
        iat: 1700000000,
        exp: None,
        verifier_id: ar4si::maps::VerifierId {
            developer: "test-dev".to_string(),
            build: "test-build".to_string(),
        },
        raw_evidence: None,
        submods: EarSubmods(submods),
        nonce: None,
        other: None,
    }
}

// ===========================================================================
// draft-ietf-rats-ear-03 Figure 5 — Contraindicated appraisal
// ===========================================================================

/// CBOR diagnostic from draft-ietf-rats-ear-03 Figure 5:
/// ```text
/// {
///   265: "tag:ietf.org,2026:rats/ear#03",
///   6: 1666529184,
///   1004: { 0: "https://veraison-project.org", 1: "vts 0.0.1" },
///   1002: h'6C696665626F61746D616E',
///   266: {
///     "PSA": {
///       1000: 96,
///       1001: { 0: 2, 2: 96, 4: 2 },
///       1003: [ "https://veraison.example/policy/1/60a0068d" ]
///     }
///   }
/// }
/// ```
#[test]
fn ear_draft_fig5_decode() {
    let bytes = hex!(
        "a5190109781d7461673a696574662e6f72672c323032363a726174732f65"
        "6172233033061a635537a01903eca200781c68747470733a2f2f76657261"
        "69736f6e2d70726f6a6563742e6f7267016976747320302e302e311903ea"
        "4b6c696665626f61746d616e19010aa163505341a31903e818601903e9a3"
        "000202186004021903eb81782a68747470733a2f2f7665726169736f6e2e"
        "6578616d706c652f706f6c6963792f312f3630613030363864"
    );
    let decoded: EarCbor = from_reader(bytes.as_slice()).unwrap();
    let ear: Ear = (&decoded).try_into().unwrap();

    assert_eq!(ear.profile, "tag:ietf.org,2026:rats/ear#03");
    assert_eq!(ear.iat, 1666529184);
    assert_eq!(ear.status, None);
    assert_eq!(ear.exp, None);
    assert_eq!(ear.verifier_id.developer, "https://veraison-project.org");
    assert_eq!(ear.verifier_id.build, "vts 0.0.1");
    assert_eq!(ear.raw_evidence.as_deref(), Some(b"lifeboatman".as_slice()));

    let psa = ear.submods.0.get("PSA").expect("PSA submod should exist");
    assert_eq!(psa.status, TrustworthinessTier::Contraindicated);

    let tv = psa
        .trustworthiness_vector
        .as_ref()
        .expect("trustworthiness_vector should exist");
    assert_eq!(tv.instance_identity, Some(2));
    assert_eq!(tv.executables, Some(96));
    assert_eq!(tv.hardware, Some(2));
    assert_eq!(tv.configuration, None);

    let pids = psa
        .appraisal_policy_ids
        .as_ref()
        .expect("appraisal_policy_ids should exist");
    assert_eq!(pids.0, vec!["https://veraison.example/policy/1/60a0068d"]);
}

#[test]
fn ear_draft_fig5_roundtrip() {
    let bytes = hex!(
        "a5190109781d7461673a696574662e6f72672c323032363a726174732f65"
        "6172233033061a635537a01903eca200781c68747470733a2f2f76657261"
        "69736f6e2d70726f6a6563742e6f7267016976747320302e302e311903ea"
        "4b6c696665626f61746d616e19010aa163505341a31903e818601903e9a3"
        "000202186004021903eb81782a68747470733a2f2f7665726169736f6e2e"
        "6578616d706c652f706f6c6963792f312f3630613030363864"
    );
    let decoded: EarCbor = from_reader(bytes.as_slice()).unwrap();
    let mut buf = vec![];
    into_writer(&decoded, &mut buf).unwrap();
    let redecoded: EarCbor = from_reader(buf.as_slice()).unwrap();
    let ear1: Ear = (&decoded).try_into().unwrap();
    let ear2: Ear = (&redecoded).try_into().unwrap();
    assert_eq!(ear1, ear2);
}

// ===========================================================================
// Veraison rust-ear test vector — basic EAR (indefinite-length maps)
// ===========================================================================

/// From veraison/rust-ear src/ear.rs test module.
/// Uses Veraison profile "tag:github.com,2023:veraison/ear" and
/// indefinite-length CBOR maps.
#[test]
fn veraison_basic_ear_decode() {
    let bytes = hex!(
        "bf190109782074 61673a6769746875 622e636f6d2c3230"
        "32333a7665726169 736f6e2f65617206 1a635537a0190 3ec"
        "a200781c68747470 733a2f2f76657261 69736f6e2d70726f"
        "6a6563742e6f7267 016a767374732030 2e302e3119010aa1"
        "647465 7374bf1903e800ff 1903ea4f37343732"
        "3639373336353633 37340aff"
    );
    let decoded: EarCbor = from_reader(bytes.as_slice()).unwrap();
    let ear: Ear = (&decoded).try_into().unwrap();

    assert_eq!(ear.profile, "tag:github.com,2023:veraison/ear");
    assert_eq!(ear.iat, 1666529184);
    assert_eq!(ear.verifier_id.developer, "https://veraison-project.org");
    assert_eq!(ear.verifier_id.build, "vsts 0.0.1");

    let test_sub = ear.submods.0.get("test").expect("test submod should exist");
    assert_eq!(test_sub.status, TrustworthinessTier::None);

    assert_eq!(
        ear.raw_evidence.as_deref(),
        Some(b"74726973656374\n".as_slice())
    );
}

// ===========================================================================
// Self-constructed roundtrip tests
// ===========================================================================

#[test]
fn minimal_ear_roundtrip() {
    let ear = minimal_ear();
    let cbor: EarCbor = (&ear).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: EarCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: Ear = (&decoded).try_into().unwrap();
    assert_eq!(ear, roundtripped);
}

#[test]
fn ear_with_all_optional_fields_roundtrip() {
    let mut submods = BTreeMap::new();
    submods.insert(
        "platform".to_string(),
        EarAppraisal {
            profile: None,
            status: TrustworthinessTier::Affirming,
            trustworthiness_vector: Some(TrustworthinessVector {
                instance_identity: Some(2),
                configuration: Some(2),
                executables: Some(2),
                file_system: Some(2),
                hardware: Some(2),
                runtime_opaque: Some(2),
                storage_opaque: Some(2),
                sourced_data: Some(2),
            }),
            appraisal_policy_ids: Some(AppraisalPolicyIds(vec![
                "policy://acme/platform/v1".to_string(),
            ])),
            nonce: None,
            other: None,
        },
    );
    submods.insert(
        "firmware".to_string(),
        EarAppraisal {
            profile: None,
            status: TrustworthinessTier::Warning,
            trustworthiness_vector: Some(TrustworthinessVector {
                instance_identity: None,
                configuration: None,
                executables: Some(32),
                file_system: None,
                hardware: None,
                runtime_opaque: None,
                storage_opaque: None,
                sourced_data: None,
            }),
            appraisal_policy_ids: None,
            nonce: None,
            other: None,
        },
    );

    let ear = Ear {
        profile: "tag:ietf.org,2026:rats/ear#03".to_string(),
        status: Some(TrustworthinessTier::Affirming),
        iat: 1700000000,
        exp: Some(1700086400),
        verifier_id: ar4si::maps::VerifierId {
            developer: "acme-verifier".to_string(),
            build: "v1.0.0".to_string(),
        },
        raw_evidence: Some(vec![0xde, 0xad, 0xbe, 0xef]),
        submods: EarSubmods(submods),
        nonce: Some(common::NonceType::One(common::BytesType(vec![0u8; 16]))),
        other: None,
    };
    let cbor: EarCbor = (&ear).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: EarCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: Ear = (&decoded).try_into().unwrap();
    assert_eq!(ear, roundtripped);
}

#[test]
fn ear_with_expiry_roundtrip() {
    let mut ear = minimal_ear();
    ear.exp = Some(1700086400);
    let cbor: EarCbor = (&ear).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: EarCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: Ear = (&decoded).try_into().unwrap();
    assert_eq!(ear, roundtripped);
}

#[test]
fn ear_with_raw_evidence_roundtrip() {
    let mut ear = minimal_ear();
    ear.raw_evidence = Some(b"some-attestation-evidence".to_vec());
    let cbor: EarCbor = (&ear).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: EarCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: Ear = (&decoded).try_into().unwrap();
    assert_eq!(ear, roundtripped);
}

#[test]
fn ear_multiple_submods_roundtrip() {
    let mut submods = BTreeMap::new();
    for name in ["attester-a", "attester-b", "attester-c"] {
        submods.insert(
            name.to_string(),
            EarAppraisal {
                profile: None,
                status: TrustworthinessTier::Affirming,
                trustworthiness_vector: None,
                appraisal_policy_ids: None,
                nonce: None,
                other: None,
            },
        );
    }
    let mut ear = minimal_ear();
    ear.submods = EarSubmods(submods);
    let cbor: EarCbor = (&ear).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: EarCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: Ear = (&decoded).try_into().unwrap();
    assert_eq!(ear, roundtripped);
}

#[test]
fn ear_appraisal_all_tiers() {
    for tier in [
        TrustworthinessTier::None,
        TrustworthinessTier::Affirming,
        TrustworthinessTier::Warning,
        TrustworthinessTier::Contraindicated,
    ] {
        let appraisal = EarAppraisal {
            profile: None,
            status: tier,
            trustworthiness_vector: None,
            appraisal_policy_ids: None,
            nonce: None,
            other: None,
        };
        let cbor: EarAppraisalCbor = (&appraisal).try_into().unwrap();
        let mut buf = vec![];
        into_writer(&cbor, &mut buf).unwrap();
        let decoded: EarAppraisalCbor = from_reader(buf.as_slice()).unwrap();
        let roundtripped: EarAppraisal = (&decoded).try_into().unwrap();
        assert_eq!(appraisal, roundtripped);
    }
}

#[test]
fn ear_appraisal_with_policy_ids_roundtrip() {
    let appraisal = EarAppraisal {
        profile: None,
        status: TrustworthinessTier::Affirming,
        trustworthiness_vector: None,
        appraisal_policy_ids: Some(AppraisalPolicyIds(vec![
            "policy://acme/v1".to_string(),
            "policy://acme/v2".to_string(),
        ])),
        nonce: None,
        other: None,
    };
    let cbor: EarAppraisalCbor = (&appraisal).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: EarAppraisalCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: EarAppraisal = (&decoded).try_into().unwrap();
    assert_eq!(appraisal, roundtripped);
}
