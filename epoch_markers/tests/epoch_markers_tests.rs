use ciborium::{de::from_reader, ser::into_writer, tag::Required};
use hex_literal::hex;

use common::OidType;
use epoch_markers::{
    arrays::{EpochTickList, MessageImprint, MessageImprintCbor},
    choices::{CborTime, EpochMarker, EpochTick},
    maps::{
        OidChoice, ProfiledEtime, ProfiledEtimeCbor, TstInfoCborTimeTag, TstInfoCborTimeTagCbor,
    },
};
use etime::{choices::BaseTime, maps::EtimeMap};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_etime(secs: i64) -> EtimeMap {
    EtimeMap {
        base_time: BaseTime::IntegerSecs(secs),
        tz_hint_critical: None,
        suffix_critical: None,
        timescale_critical: None,
        timescale_legacy: None,
        clock_class: None,
        milliseconds: None,
        clock_accuracy: None,
        offset_scaled_log_variance: None,
        microseconds: None,
        uncertainty: None,
        guarantee: None,
        nanoseconds: None,
        tz_hint_elective: None,
        suffix_elective: None,
        picoseconds: None,
        timescale_elective: None,
        femtoseconds: None,
        attoseconds: None,
        other: vec![],
    }
}

// ===========================================================================
// ProfiledEtime tests
// ===========================================================================

#[test]
fn profiled_etime_requires_key1_integer() {
    let pe = ProfiledEtime(minimal_etime(1700000000));
    assert!(pe.validate().is_ok());
}

#[test]
fn profiled_etime_requires_key1_float() {
    let mut m = minimal_etime(0);
    m.base_time = BaseTime::FloatSecs(1700000000.5);
    let pe = ProfiledEtime(m);
    assert!(pe.validate().is_ok());
}

#[test]
fn profiled_etime_requires_key1_not_key4() {
    let mut m = minimal_etime(0);
    m.base_time = BaseTime::DecFrac(-3, 1700000000500);
    let pe = ProfiledEtime(m);
    assert!(pe.validate().is_err());
}

#[test]
fn profiled_etime_requires_key1_not_key5() {
    let mut m = minimal_etime(0);
    m.base_time = BaseTime::BigFloat(-10, 1700000000);
    let pe = ProfiledEtime(m);
    assert!(pe.validate().is_err());
}

#[test]
fn profiled_etime_cbor_roundtrip() {
    let pe = ProfiledEtimeCbor(Required(minimal_etime(1700000000)));
    let mut buf = vec![];
    into_writer(&pe, &mut buf).unwrap();
    let decoded: ProfiledEtimeCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(pe, decoded);
}

// ===========================================================================
// draft-ietf-rats-epoch-markers-03 Figure 3/4 — etime epoch marker
// ===========================================================================

/// Figure 4 hex encoding of an etime epoch marker:
/// `1001({1: 851042397, -10: "America/Los_Angeles", -11: {"u-ca": "hebrew"}})`
#[test]
fn draft_fig4_etime_epoch_marker_decode() {
    let bytes = hex!(
        "d903e9a3011a32b9e05d2973416d65726963612f4c6f735f416e67656c6573"
        "2aa164752d636166686562726577"
    );
    let decoded: EpochMarker = from_reader(bytes.as_slice()).unwrap();
    match &decoded {
        EpochMarker::CborTime(CborTime::Etime(pe)) => {
            assert_eq!(pe.0.0.base_time, BaseTime::IntegerSecs(851042397));
            assert_eq!(
                pe.0.0.tz_hint_elective.as_deref(),
                Some("America/Los_Angeles")
            );
        }
        other => panic!("expected CborTime::Etime, got {:?}", other),
    }
}

#[test]
fn draft_fig4_etime_roundtrip() {
    let bytes = hex!(
        "d903e9a3011a32b9e05d2973416d65726963612f4c6f735f416e67656c6573"
        "2aa164752d636166686562726577"
    );
    let decoded: EpochMarker = from_reader(bytes.as_slice()).unwrap();
    let mut buf = vec![];
    into_writer(&decoded, &mut buf).unwrap();
    let redecoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded, redecoded);
}

// ===========================================================================
// CborTime variants
// ===========================================================================

#[test]
fn cbor_time_tdate_roundtrip() {
    // #6.0("2024-01-15T12:00:00Z")
    let ct = CborTime::Tdate(Required("2024-01-15T12:00:00Z".to_string()));
    let marker = EpochMarker::CborTime(ct);
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

#[test]
fn cbor_time_int_roundtrip() {
    // #6.1(1700000000)
    let ct = CborTime::TimeInt(Required(1700000000));
    let marker = EpochMarker::CborTime(ct);
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

// ===========================================================================
// EpochTick / EpochTickList
// ===========================================================================

#[test]
fn epoch_tick_text_roundtrip() {
    let tick = EpochTick::Text("tick-abc-123".to_string());
    let marker = EpochMarker::EpochTick(Required(tick));
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

#[test]
fn epoch_tick_bytes_roundtrip() {
    let tick = EpochTick::Bytes(vec![0xde, 0xad, 0xbe, 0xef]);
    let marker = EpochMarker::EpochTick(Required(tick));
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

#[test]
fn epoch_tick_int_roundtrip() {
    let tick = EpochTick::Int(42);
    let marker = EpochMarker::EpochTick(Required(tick));
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

#[test]
fn epoch_tick_list_roundtrip() {
    let list = EpochTickList(vec![
        EpochTick::Text("tick-1".to_string()),
        EpochTick::Int(99),
        EpochTick::Bytes(vec![0x01, 0x02]),
    ]);
    let marker = EpochMarker::EpochTickList(Required(list));
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

// ===========================================================================
// StrictlyMonotonicCounter
// ===========================================================================

#[test]
fn strictly_monotonic_counter_roundtrip() {
    let marker = EpochMarker::StrictlyMonotonicCounter(Required(12345));
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

// ===========================================================================
// ClassicalTstInfo
// ===========================================================================

#[test]
fn classical_tst_info_roundtrip() {
    let der_bytes = vec![0x30, 0x82, 0x01, 0x00]; // placeholder DER
    let marker = EpochMarker::ClassicalTstInfo(Required(serde_bytes::ByteBuf::from(der_bytes)));
    let mut buf = vec![];
    into_writer(&marker, &mut buf).unwrap();
    let decoded: EpochMarker = from_reader(buf.as_slice()).unwrap();
    assert_eq!(marker, decoded);
}

// ===========================================================================
// TstInfoCborTimeTag
// ===========================================================================

#[test]
fn tst_info_cbor_time_tag_roundtrip() {
    let sha256_hash = hex!("BF4EE9143EF2329B1B778974AAD445064940B9CAE373C9E35A7B23361282698F");
    let tst = TstInfoCborTimeTag {
        version: 1,
        policy: OidChoice::AbsoluteOid(Required(OidType(
            hex!("608648016503040201").to_vec(), // sha-256 OID
        ))),
        message_imprint: MessageImprint {
            hash_alg: -16, // sha-256 in COSE
            hash_value: sha256_hash.to_vec(),
        },
        serial_number: 1,
        etime: ProfiledEtime(minimal_etime(1700000000)),
        ordering: Some(false),
        nonce: Some(12345),
        tsa: None,
        other: None,
    };
    let cbor: TstInfoCborTimeTagCbor = (&tst).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: TstInfoCborTimeTagCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: TstInfoCborTimeTag = (&decoded).try_into().unwrap();
    assert_eq!(tst, roundtripped);
}

#[test]
fn tst_info_cbor_time_tag_minimal_roundtrip() {
    let tst = TstInfoCborTimeTag {
        version: 1,
        policy: OidChoice::AbsoluteOid(Required(OidType(vec![0x60, 0x86, 0x48]))),
        message_imprint: MessageImprint {
            hash_alg: -16,
            hash_value: vec![0u8; 32],
        },
        serial_number: 42,
        etime: ProfiledEtime(minimal_etime(851042397)),
        ordering: None,
        nonce: None,
        tsa: None,
        other: None,
    };
    let cbor: TstInfoCborTimeTagCbor = (&tst).try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();
    let decoded: TstInfoCborTimeTagCbor = from_reader(buf.as_slice()).unwrap();
    let roundtripped: TstInfoCborTimeTag = (&decoded).try_into().unwrap();
    assert_eq!(tst, roundtripped);
}

// ===========================================================================
// MessageImprint
// ===========================================================================

#[test]
fn message_imprint_roundtrip() {
    let mi = MessageImprintCbor {
        hash_alg: -16,
        hash_value: vec![0xab; 32],
    };
    let mut buf = vec![];
    into_writer(&mi, &mut buf).unwrap();
    let decoded: MessageImprintCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(mi, decoded);
}

// ===========================================================================
// OidChoice
// ===========================================================================

#[test]
fn oid_choice_absolute_roundtrip() {
    let oid = OidChoice::AbsoluteOid(Required(OidType(vec![0x60, 0x86, 0x48])));
    let mut buf = vec![];
    into_writer(&oid, &mut buf).unwrap();
    let decoded: OidChoice = from_reader(buf.as_slice()).unwrap();
    assert_eq!(oid, decoded);
}

#[test]
fn oid_choice_relative_roundtrip() {
    let oid = OidChoice::RelativeOid(Required(OidType(vec![0x01, 0x02, 0x03])));
    let mut buf = vec![];
    into_writer(&oid, &mut buf).unwrap();
    let decoded: OidChoice = from_reader(buf.as_slice()).unwrap();
    assert_eq!(oid, decoded);
}
