//! Tests based on external EAT test vectors.
//!
//! Sources:
//! - RFC 9711 Appendix A examples (canonical spec examples)
//! - IETF RATS WG eat repo (cddl/Example-Payloads/)
//! - Laurence Lundblade's EAT-test-vectors (IETF 111 hackathon)
//! - veraison/eat Go library test vectors

extern crate alloc;
use alloc::collections::BTreeMap;

use ciborium::{de::from_reader, ser::into_writer, value::Value};
use hex_literal::hex;

use eat::{
    cbor_specific::{SelectorCbor, SubmodsMapCbor, SubmoduleCbor},
    choices::{DebugStatusType, Oemid},
    maps::{ClaimsSetClaims, ClaimsSetClaimsCbor, LocationTypeCbor},
};

// ===========================================================================
// RFC 9711 Appendix A — Claims-Set Payloads
// ===========================================================================

/// RFC 9711 A.1.3 / A.2.1 — "EAT Produced by an Attestation Hardware Block"
///
/// 47-byte token: nonce, ueid, oemid (PEN 64242), oemboot, dbgstat, hwversion.
/// Hex extracted from A.2.1 CWT payload.
#[test]
fn rfc9711_a1_3_hw_block() {
    let data = hex!(
        "A60A4CD79B964DDD5471C1393C8888190100500198F50A4FF6C05861C8860D13A638EA"
        "19010219FAF2190106F5190107031901048263332E3101"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();

    assert!(csc.nonce.is_some());
    assert!(csc.ueid.is_some());
    assert!(csc.oemid.is_some());
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::DisabledPermanently));
    assert!(csc.hardware_version.is_some());
    assert_eq!(csc.hardware_version.as_ref().unwrap().version, "3.1");

    // CBOR → JSON → CBOR roundtrip
    let json: ClaimsSetClaims = csc.clone().try_into().unwrap();
    let cbor2: ClaimsSetClaimsCbor = json.try_into().unwrap();
    assert_eq!(csc, cbor2);

    // Encode → decode roundtrip
    let mut buf = vec![];
    let _ = into_writer(&csc, &mut buf);
    let decoded: ClaimsSetClaimsCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(csc, decoded);
}

/// RFC 9711 A.1.1 — "Simple TEE Attestation"
///
/// nonce, oemboot, dbgstat, manifests (with embedded payload CoSWID).
#[test]
fn rfc9711_a1_1_simple_tee() {
    let data = hex!(
        "A40A5048DF7B172D70B5A18935D0460A73DD71190106F51901070219011081821901025858"
        "A60064336132340C01016B41636D6520544545204F530D65332E312E340282A2181F6B4163"
        "6D6520544545204F53182101A2181F6B41636D6520544545204F5318210206A111A118186E"
        "61636D655F7465655F332E657865"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();

    assert!(csc.nonce.is_some());
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::DisabledSinceBoot));
    assert!(csc.manifests.is_some());
    assert_eq!(csc.manifests.as_ref().unwrap().0.len(), 1);

    let json: ClaimsSetClaims = csc.clone().try_into().unwrap();
    let cbor2: ClaimsSetClaimsCbor = json.try_into().unwrap();
    assert_eq!(csc, cbor2);
}

/// RFC 9711 A.1.2 — "Submodules for Board and Device"
///
/// Two named Claims-Set submodules ("board", "device"). Exercises oemid
/// (IEEE OUI), hwmodel, hwversion, swname, swversion, iat (tag 1).
#[test]
fn rfc9711_a1_2_submods() {
    let data = hex!(
        "AB0A50E253CABEDC9EEC24AC4E25BCBEAF7765190100500198F50A4FF6C05861C8860D"
        "13A638EA1901024389482319010350549DCECC8B987C737B44E40F7C635CE819010482"
        "65312E332E340119010E6741636D65204F5319010F8265332E352E3501190106F51901"
        "070306C11A5AFD322E19010AA265626F617264A3190102509BEF8787EBA13E2C8F6E7C"
        "B4B1F4619A19010350EE80F5A66C1FB9742999A8FDAB930893190104826432"
        "2E30610266646576696365A219010219EF321901048263342E3001"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();

    assert!(csc.nonce.is_some());
    assert!(csc.ueid.is_some());
    assert!(csc.oemid.is_some());
    assert!(csc.hardware_model.is_some());
    assert_eq!(csc.sw_name, Some("Acme OS".to_string()));
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::DisabledPermanently));
    assert!(csc.iat.is_some());

    // submods map
    let submods = csc.submods.as_ref().unwrap();
    assert_eq!(submods.0.len(), 2);

    // board: oemid (IEEE), hwmodel, hwversion
    match &submods.0["board"] {
        SubmoduleCbor::ClaimsSet(cs) => {
            assert!(cs.oemid.is_some());
            assert!(cs.hardware_model.is_some());
            assert!(cs.hardware_version.is_some());
        }
        _ => panic!("Expected ClaimsSet for 'board'"),
    }

    // device: oemid (PEN 61234), hwversion
    match &submods.0["device"] {
        SubmoduleCbor::ClaimsSet(cs) => {
            assert!(cs.oemid.is_some());
            assert!(cs.hardware_version.is_some());
        }
        _ => panic!("Expected ClaimsSet for 'device'"),
    }

    let json: ClaimsSetClaims = csc.clone().try_into().unwrap();
    let cbor2: ClaimsSetClaimsCbor = json.try_into().unwrap();
    assert_eq!(csc, cbor2);
}

/// RFC 9711 A.1.4 — "Key / Key Store Attestation"
///
/// Exercises: nonce (24 bytes), oemboot, dbgstat, manifests, exp, iat,
/// private claims (-80000, -80001), and submods with "HLOS" submodule.
#[test]
fn rfc9711_a1_4_key_store() {
    let data = hex!(
        "A90A581899B67438DBA40743266F70BF75FEB1026D513497A229BFE8190106F5190107"
        "021901108182190102583CA6006837626233343837660C000169436172626F6E697465"
        "0D63312E320E0102A2181F75496E647573747269616C204175746F6D6174696F6E1821"
        "0204C11A6169CF3206C11A6169B3183A0001387F6B66696E6765727072696E743A0001"
        "3880A50102025036675C206F96236C3F51F54637B94CED200221582065EDA5A12577C2"
        "BAE829437FE338701A10AAA375E1BB5B5DE108DE439C08551D2258201E52ED75701163"
        "F7F9E40DDF9F341B3DC9BA860AF7E0CA7CA7E9EECD0084D19C19010AA164484C4F53A3"
        "0A488B0B28782A23D3F6190106F51901108182190102583DA600687337653734"
        "6B78380C00016844726F6964204F530D6552322E44320E0302A2181F75496E64757374"
        "7269616C204175746F6D6174696F6E182102"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();

    assert!(csc.nonce.is_some());
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::DisabledSinceBoot));
    assert!(csc.manifests.is_some());
    assert!(csc.exp.is_some());
    assert!(csc.iat.is_some());

    // Private claims land in 'other'
    assert!(csc.other.is_some());
    let other = csc.other.as_ref().unwrap();
    assert!(other.len() >= 2); // -80000 and -80001

    // HLOS submodule
    let submods = csc.submods.as_ref().unwrap();
    assert_eq!(submods.0.len(), 1);
    match &submods.0["HLOS"] {
        SubmoduleCbor::ClaimsSet(cs) => {
            assert!(cs.nonce.is_some());
            assert_eq!(cs.oem_boot, Some(true));
            assert!(cs.manifests.is_some());
        }
        _ => panic!("Expected ClaimsSet for 'HLOS'"),
    }
}

/// RFC 9711 A.1.5 — "Software Measurements of an IoT Device"
///
/// Exercises: nonce, oemboot, dbgstat, oemid (IEEE CID), ueid, and submods
/// with "OS" submodule containing measurements (evidence CoSWID).
#[test]
fn rfc9711_a1_5_iot_measurements() {
    let data = hex!(
        "a60a485e19fba4483c7896190106f519010702190102438945ad190100500198f50a4f"
        "f6c05861c8860d13a638ea19010aa1624f53a3190106f5190107021901118182190102"
        "58f4a600663463613234350c17016d41636d6520522d496f542d4f530d65332e312e34"
        "02a2181f7241636d65204261736520417474657374657218210103a11183a318187161"
        "636d655f725f696f745f6f732e657865141a0044b349078201582005f6b327c173b419"
        "2bd2c3ec248a292215eab456611bf7a783e25c1782479905a318186d7265736f757263"
        "65732e727363141a000c38b10782015820c142b9aba4280c4bb8c75f716a43c9952669"
        "4caabe529571f5569bb7dc542f98a318186a636f6d6d6f6e2e6c6962141a00233d3b07"
        "82015820a6a9dcdfb3884da5f884e4e1e8e8629958c2dbc702741443a913e34de9333b"
        "e6"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();

    assert!(csc.nonce.is_some());
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::DisabledSinceBoot));
    assert!(csc.oemid.is_some());
    assert!(csc.ueid.is_some());

    // "OS" submodule with measurements
    let submods = csc.submods.as_ref().unwrap();
    assert_eq!(submods.0.len(), 1);
    match &submods.0["OS"] {
        SubmoduleCbor::ClaimsSet(cs) => {
            assert_eq!(cs.oem_boot, Some(true));
            assert_eq!(cs.debug_status, Some(DebugStatusType::DisabledSinceBoot));
            assert!(cs.measurements.is_some());
        }
        _ => panic!("Expected ClaimsSet for 'OS'"),
    }
}

// ===========================================================================
// IETF RATS WG eat repo — Example-Payloads
// ===========================================================================

/// IETF RATS WG minimal.diag — smallest valid EAT (nonce + oemboot)
#[test]
fn ietf_rats_minimal() {
    let data = hex!("A20A48948F8860D13A463E190106F5");
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    assert!(csc.nonce.is_some());
    assert_eq!(csc.oem_boot, Some(true));
    assert!(csc.iss.is_none());
    assert!(csc.ueid.is_none());
    assert!(csc.submods.is_none());
    assert!(csc.location.is_none());
}

/// IETF RATS WG simple.diag — issuer, nonce, ueid, oemid (IEEE), hwmodel,
/// oemboot, dbgstat, iat (tag 1)
#[test]
fn ietf_rats_simple() {
    let data = hex!(
        "A80178186A6F655F61747465737465722E6578616D706C652E636F6D0A48948F8860"
        "D13A463E190100500198F50A4FF6C05861C8860D13A638EA190102438948231901"
        "0350549DCECC8B987C737B44E40F7C635CE8190106F519010703"
        "06C11A5AFD322E"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    assert_eq!(csc.iss, Some("joe_attester.example.com".to_string()));
    assert!(csc.nonce.is_some());
    assert!(csc.ueid.is_some());
    assert!(csc.oemid.is_some());
    assert!(csc.hardware_model.is_some());
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::DisabledPermanently));
    assert!(csc.iat.is_some());

    let json: ClaimsSetClaims = csc.clone().try_into().unwrap();
    let cbor2: ClaimsSetClaimsCbor = json.try_into().unwrap();
    assert_eq!(csc, cbor2);
}

// ===========================================================================
// veraison/eat — Go library test vectors
// ===========================================================================

/// veraison/eat TestEat_Full_RoundtripCBOR — full token with all CWT claims,
/// nonce, ueid, oemid, uptime, oemboot, dbgstat (enabled), and location
/// (float64 lat/lon).
///
/// Note: veraison uses a 6-byte oemid which is invalid per RFC 9711
/// (must be 3 bytes for IEEE or 16 bytes for Random). We test a corrected
/// version with a valid 3-byte IEEE oemid.
#[test]
fn veraison_full_eat() {
    let data = hex!(
        "AE016941636D6520496E632E026772722D74726170036941636D6520496E632E04C100"
        "05C10006C10007 46FFFFFFFFFFFF 0A48 0000000000000000"
        "190100 5101DEADBEEFDEADBEEFDEADBEEFDEADBEEF"
        "190102 43FFFFFF 190105183C 190106F5 19010701"
        "190108 A201FB4028AE147AE147AE 02FB404C63D70A3D70A4"
    );
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();

    assert_eq!(csc.iss, Some("Acme Inc.".to_string()));
    assert_eq!(csc.sub, Some("rr-trap".to_string()));
    assert_eq!(csc.aud, Some("Acme Inc.".to_string()));
    assert!(csc.exp.is_some());
    assert!(csc.nbf.is_some());
    assert!(csc.iat.is_some());
    assert!(csc.cti.is_some());
    assert!(csc.nonce.is_some());
    assert!(csc.ueid.is_some());
    assert!(csc.oemid.is_some());
    assert_eq!(csc.uptime, Some(60));
    assert_eq!(csc.oem_boot, Some(true));
    assert_eq!(csc.debug_status, Some(DebugStatusType::Disabled));

    // Location with float64 coordinates
    assert!(csc.location.is_some());
    let loc = csc.location.as_ref().unwrap();
    assert!((loc.latitude - 12.34).abs() < 0.001);
    assert!((loc.longitude - 56.78).abs() < 0.001);

    // Encode → decode roundtrip
    let mut buf = vec![];
    let _ = into_writer(&csc, &mut buf);
    let decoded: ClaimsSetClaimsCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(csc, decoded);
}

/// veraison/eat original 6-byte oemid — rejects invalid length
#[test]
fn veraison_oemid_6_bytes_rejected() {
    // {258: h'FFFFFFFFFFFF'} — 6 bytes, not 3 or 16
    let data = hex!("A119010246FFFFFFFFFFFF");
    let result: Result<ClaimsSetClaimsCbor, _> = from_reader(data.as_slice());
    assert!(result.is_err());
}

/// veraison/eat location — float64 lat/lon only
#[test]
fn veraison_location_float64() {
    let data = hex!("A201FB4028AE147AE147AE02FB404C63D70A3D70A4");
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    assert!((loc.latitude - 12.34).abs() < 0.001);
    assert!((loc.longitude - 56.78).abs() < 0.001);
    assert!(loc.altitude.is_none());
}

/// veraison/eat location — float16 lat/lon
#[test]
fn veraison_location_float16() {
    let data = hex!("A201F93FB902F94002");
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    // f16 precision is limited; just check reasonable range
    assert!(loc.latitude > 0.9 && loc.latitude < 2.0);
    assert!(loc.longitude > 1.9 && loc.longitude < 3.0);
}

/// veraison/eat location — float32 lat/lon
#[test]
fn veraison_location_float32() {
    let data = hex!("A201FA40490E5602FAC141EB85");
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    #[allow(clippy::approx_constant)]
    let expected_lat = 3.1415; // veraison test value, not pi
    assert!((loc.latitude - expected_lat).abs() < 0.001);
    assert!((loc.longitude - (-12.12)).abs() < 0.01);
}

/// veraison/eat location — integer lat/lon (CDDL `number` includes int)
#[test]
fn veraison_location_integer() {
    // lat=3 (uint), lon=-12 (nint, encoded as 0x2b = -12)
    let data: Vec<u8> = vec![0xA2, 0x01, 0x03, 0x02, 0x2B];
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    assert_eq!(loc.latitude, 3.0);
    assert_eq!(loc.longitude, -12.0);
}

/// veraison/eat location — all fields populated
///
/// Note: LocationTypeCbor is a StructToMap with integer keys. This hex
/// encodes a standalone location-type map, not wrapped in a Claims-Set.
#[test]
fn veraison_location_all_fields() {
    // Construct via the struct directly since the raw veraison hex encodes
    // a standalone location map (which our serde deserializer handles).
    let data: Vec<u8> = vec![
        0xA9, 0x01, 0x03, 0x02, 0xFB, 0xC0, 0x28, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x03, 0x01,
        0x04, 0x01, 0x05, 0x01, 0x06, 0x01, 0x07, 0x01, 0x08, 0xC1, 0x1A, 0x5F, 0xA9, 0xD8, 0x00,
        0x09, 0x19, 0x03, 0xC9,
    ];
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    assert_eq!(loc.latitude, 3.0);
    assert!((loc.longitude - (-12.1)).abs() < 0.01);
    assert_eq!(loc.altitude, Some(1.0));
    assert_eq!(loc.accuracy, Some(1.0));
    assert_eq!(loc.altitude_accuracy, Some(1.0));
    assert_eq!(loc.heading, Some(1.0));
    assert_eq!(loc.speed, Some(1.0));
    assert!(loc.timestamp.is_some());
    assert_eq!(loc.age, Some(969));
}

/// veraison/eat location — unmarshal uint lat/lon
#[test]
fn veraison_location_uint() {
    // lat=12, lon=56
    let data = hex!("A2010C021838");
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    assert_eq!(loc.latitude, 12.0);
    assert_eq!(loc.longitude, 56.0);
}

/// veraison/eat location — unmarshal negative int lat/lon
#[test]
fn veraison_location_nint() {
    // lat=-12 (0x2B), lon=-56 (0x3837)
    let data = hex!("A2012B023837");
    let loc: LocationTypeCbor = from_reader(data.as_slice()).unwrap();
    assert_eq!(loc.latitude, -12.0);
    assert_eq!(loc.longitude, -56.0);
}

/// veraison/eat submods — CBOR roundtrip with Claims-Set and token submodules
#[test]
fn veraison_submods_simple() {
    // { "0": {10: h'0000000000000000'}, "xyz": h'D83DD241A0' }
    let data = hex!("A2 6130 A10A480000000000000000 6378797A 45D83DD241A0");
    let sm: SubmodsMapCbor =
        Value::try_into(from_reader::<Value, _>(data.as_slice()).unwrap()).unwrap();
    assert_eq!(sm.0.len(), 2);
    assert!(matches!(sm.0["0"], SubmoduleCbor::ClaimsSet(_)));
    assert!(matches!(
        sm.0["xyz"],
        SubmoduleCbor::SelectorCbor(SelectorCbor::CborTokenInsideCborToken(_))
    ));
}

/// veraison/eat submods — nested submods (submods within a Claims-Set submod)
#[test]
fn veraison_submods_nested() {
    // { "0": { 266: { "xyz": h'D83DD241A0' } } }
    let data = hex!("A1 6130 A119010AA1 6378797A 45D83DD241A0");
    let sm: SubmodsMapCbor =
        Value::try_into(from_reader::<Value, _>(data.as_slice()).unwrap()).unwrap();
    assert_eq!(sm.0.len(), 1);
    match &sm.0["0"] {
        SubmoduleCbor::ClaimsSet(cs) => {
            assert!(cs.submods.is_some());
            let inner = cs.submods.as_ref().unwrap();
            assert_eq!(inner.0.len(), 1);
            assert!(matches!(
                inner.0["xyz"],
                SubmoduleCbor::SelectorCbor(SelectorCbor::CborTokenInsideCborToken(_))
            ));
        }
        _ => panic!("Expected ClaimsSet"),
    }
}

// ===========================================================================
// Hackathon / ctoken — valid vectors (using RFC 9711 labels where possible)
// ===========================================================================

/// Empty map — valid CBOR but rejected by ciborium deserializer
#[test]
fn hackathon_completely_empty() {
    let data = hex!("A0");
    let value: Value = from_reader(data.as_slice()).unwrap();
    assert!(matches!(value, Value::Map(ref m) if m.is_empty()));

    // ClaimsSetClaimsCbor rejects empty maps
    let result: Result<ClaimsSetClaimsCbor, _> = from_reader(data.as_slice());
    assert!(result.is_err());

    // Single-claim map works
    let single = hex!("A1190106F5");
    let csc: ClaimsSetClaimsCbor = from_reader(single.as_slice()).unwrap();
    assert_eq!(csc.oem_boot, Some(true));
}

/// ctoken valid profile — OID format (1.3.6.1.4.1.45984.4)
#[test]
fn ctoken_profile_valid_oid() {
    // {18: h'06092B0601040185BF1004'}  — label 18 is not the EAT profile label (265),
    // it's the CWT/PSA profile label. This will land in 'other'.
    let data = hex!("A1124B06092B0601040185BF1004");
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    assert!(csc.other.is_some());
}

/// ctoken valid profile — URI format
#[test]
fn ctoken_profile_valid_uri() {
    // {18: "http://arm.com/psa/2.0.0"}
    let data = hex!("A1127818687474703A2F2F61726D2E636F6D2F7073612F322E302E30");
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    assert!(csc.other.is_some());
}

/// ctoken valid secboot claim (label 15 is not RFC 9711's oemboot label 262)
#[test]
fn ctoken_secboot_valid() {
    let data_true = hex!("A10FF5");
    let csc: ClaimsSetClaimsCbor = from_reader(data_true.as_slice()).unwrap();
    // Label 15 is not a standard EAT label, lands in other
    assert!(csc.other.is_some());

    let data_false = hex!("A10FF4");
    let csc2: ClaimsSetClaimsCbor = from_reader(data_false.as_slice()).unwrap();
    assert!(csc2.other.is_some());
}

/// Hackathon deeply nested submods — pre-RFC labels
///
/// Uses label 20 (not 266) for submods and 2-byte nonces (not 8-64).
/// These are draft-era conventions from July 2021. Verify raw CBOR parses.
#[test]
fn hackathon_deeply_nested_submods() {
    let data = hex!(
        "A20A42000014A26473756231A20A42010114A26473756232A20A42020214A264737562"
        "33A20A42030314A26473756234A20A42040414A26473756235A10A42050566746F6B65"
        "6E3545555555555566746F6B656E3445444444444466746F6B656E3345333333333366"
        "746F6B656E3245222222222266746F6B656E31451111111111"
    );
    let value: Value = from_reader(data.as_slice()).unwrap();
    if let Value::Map(entries) = &value {
        assert_eq!(entries.len(), 2); // nonce (10) + submods (20)
    } else {
        panic!("Expected Map");
    }
}

// ===========================================================================
// Programmatic tests — SubmodsMapCbor
// ===========================================================================

/// Multiple named Claims-Set submodules, encode/decode/JSON roundtrip
#[test]
fn submods_map_multiple_entries() {
    let board = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: None,
        dloas: None,
        hardware_model: Some(vec![0xEE, 0x80]),
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        oem_boot: None,
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        measurement_results: None,
        oemid: None,
        sueids: None,
        submods: None,
        other: None,
    };
    let device = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: Some(DebugStatusType::Disabled),
        dloas: None,
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        oem_boot: Some(false),
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        measurement_results: None,
        oemid: None,
        sueids: None,
        submods: None,
        other: None,
    };

    let mut submods = BTreeMap::new();
    submods.insert(
        "board".to_string(),
        SubmoduleCbor::ClaimsSet(Box::new(board)),
    );
    submods.insert(
        "device".to_string(),
        SubmoduleCbor::ClaimsSet(Box::new(device)),
    );

    let csc = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: None,
        dloas: None,
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        oem_boot: Some(true),
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        measurement_results: None,
        oemid: None,
        sueids: None,
        submods: Some(SubmodsMapCbor(submods)),
        other: None,
    };

    let mut buf = vec![];
    let _ = into_writer(&csc, &mut buf);
    let decoded: ClaimsSetClaimsCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(csc, decoded);
    assert_eq!(decoded.submods.as_ref().unwrap().0.len(), 2);

    let json: ClaimsSetClaims = decoded.try_into().unwrap();
    let cbor2: ClaimsSetClaimsCbor = json.try_into().unwrap();
    assert_eq!(csc, cbor2);
}

/// SubmodsMapCbor with all three Submodule variants: Claims-Set (map),
/// JWT (text), CBOR token (bytes). Tests both Value-based construction
/// AND serde encode/decode roundtrip (which previously failed for bstr).
#[test]
fn submods_map_with_selectors() {
    // Build via Value-based path
    let submods_map = Value::Map(vec![
        (
            Value::Text("claims".to_string()),
            Value::Map(vec![(
                Value::Integer(1.into()),
                Value::Text("inner-issuer".to_string()),
            )]),
        ),
        (
            Value::Text("jwt".to_string()),
            Value::Text("eyJhbGciOiJub25lIn0.eyJpc3MiOiJ0ZXN0In0.".to_string()),
        ),
        (
            Value::Text("nested_cbor".to_string()),
            Value::Bytes(vec![0xD8, 0x3D, 0xA1, 0x01, 0x02]),
        ),
    ]);

    let sm: SubmodsMapCbor = submods_map.try_into().unwrap();
    assert_eq!(sm.0.len(), 3);
    assert!(matches!(sm.0["claims"], SubmoduleCbor::ClaimsSet(_)));
    assert!(matches!(
        sm.0["jwt"],
        SubmoduleCbor::SelectorCbor(SelectorCbor::JsonTokenInsideCborToken(_))
    ));
    assert!(matches!(
        sm.0["nested_cbor"],
        SubmoduleCbor::SelectorCbor(SelectorCbor::CborTokenInsideCborToken(_))
    ));

    if let SubmoduleCbor::ClaimsSet(cs) = &sm.0["claims"] {
        assert_eq!(cs.iss, Some("inner-issuer".to_string()));
    }

    // Now test serde roundtrip: wrap in a ClaimsSetClaimsCbor, encode, decode
    let csc = ClaimsSetClaimsCbor {
        iss: None,
        sub: None,
        aud: None,
        exp: None,
        nbf: None,
        iat: None,
        cti: None,
        nonce: None,
        boot_count: None,
        boot_seed: None,
        debug_status: None,
        dloas: None,
        hardware_model: None,
        hardware_version: None,
        intended_use: None,
        location: None,
        profile: None,
        oem_boot: None,
        sw_name: None,
        sw_version: None,
        ueid: None,
        uptime: None,
        manifests: None,
        measurements: None,
        measurement_results: None,
        oemid: None,
        sueids: None,
        submods: Some(sm),
        other: None,
    };

    let mut buf = vec![];
    let _ = into_writer(&csc, &mut buf);
    let decoded: ClaimsSetClaimsCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(csc, decoded);

    // Verify each variant survived the serde roundtrip
    let rt_sm = decoded.submods.as_ref().unwrap();
    assert_eq!(rt_sm.0.len(), 3);
    assert!(matches!(rt_sm.0["claims"], SubmoduleCbor::ClaimsSet(_)));
    assert!(matches!(
        rt_sm.0["jwt"],
        SubmoduleCbor::SelectorCbor(SelectorCbor::JsonTokenInsideCborToken(_))
    ));
    assert!(matches!(
        rt_sm.0["nested_cbor"],
        SubmoduleCbor::SelectorCbor(SelectorCbor::CborTokenInsideCborToken(_))
    ));
}

/// SubmodsMapCbor rejects non-text keys
#[test]
fn submods_map_rejects_non_text_key() {
    let bad = Value::Map(vec![(Value::Integer(42.into()), Value::Map(vec![]))]);
    let result: Result<SubmodsMapCbor, String> = bad.try_into();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("text"));
}

/// SubmodsMapCbor rejects non-map CBOR value
#[test]
fn submods_map_rejects_non_map() {
    let bad = Value::Text("not a map".to_string());
    let result: Result<SubmodsMapCbor, String> = bad.try_into();
    assert!(result.is_err());
}

// ===========================================================================
// OemId variants
// ===========================================================================

/// OemId PEN (integer) — from RFC 9711 A.1.3
#[test]
fn oemid_pen() {
    // {258: 64242}
    let data = hex!("A119010219FAF2");
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    match csc.oemid.as_ref().unwrap() {
        Oemid::Pen(v) => assert_eq!(*v, 64242),
        _ => panic!("Expected PEN"),
    }
}

/// OemId IEEE OUI (3 bytes) — from RFC 9711 A.1.2
#[test]
fn oemid_ieee() {
    // {258: h'894823'}
    let data = hex!("A11901024389 4823");
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    match csc.oemid.as_ref().unwrap() {
        Oemid::Ieee(v) => assert_eq!(v, &vec![0x89, 0x48, 0x23]),
        _ => panic!("Expected IEEE"),
    }
}

/// OemId Random (16 bytes)
#[test]
fn oemid_random() {
    // {258: h'0198f50a4ff6c05861c8860d13a638ea'} — 16 bytes
    let data = hex!("A1190102500198F50A4FF6C05861C8860D13A638EA");
    let csc: ClaimsSetClaimsCbor = from_reader(data.as_slice()).unwrap();
    match csc.oemid.as_ref().unwrap() {
        Oemid::Random(v) => assert_eq!(v.len(), 16),
        _ => panic!("Expected Random"),
    }
}
