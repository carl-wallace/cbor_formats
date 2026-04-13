use std::collections::BTreeMap;

use ciborium::{de::from_reader, ser::into_writer, tag::Required};
use hex_literal::hex;

use etime::{
    choices::{BaseTime, EtimeTimescale, SuffixValues},
    maps::{DurationMapCbor, EtimeMap, EtimeMapCbor, Period, PeriodCbor},
};

// ---------------------------------------------------------------------------
// Helper to build a minimal EtimeMap
// ---------------------------------------------------------------------------

fn minimal(base_time: BaseTime) -> EtimeMap {
    EtimeMap {
        base_time,
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
// RFC 9581 examples — decode from known-good CBOR and verify field values
// ===========================================================================

/// RFC 9581 Figure 4, example 1:
/// `1001({1: 1697724754, -6: 873294, -7: {1: 0, -6: 1000}})`
///
/// Represents 2023-10-19T12:12:34.873294Z with 1ms uncertainty expressed
/// as 1000 microseconds.
#[test]
fn rfc9581_fig4_example1_decode() {
    let bytes = hex!("d903e9a3011a65313952251a000d534e26a20100251903e8");
    let decoded: EtimeMapCbor = from_reader(bytes.as_slice()).unwrap();
    let m = &decoded.0;

    assert_eq!(m.base_time, BaseTime::IntegerSecs(1697724754));
    assert_eq!(m.microseconds, Some(873294));

    let unc = m
        .uncertainty
        .as_ref()
        .expect("uncertainty should be present");
    assert_eq!(unc.base_time, BaseTime::IntegerSecs(0));
    assert_eq!(unc.microseconds, Some(1000));
}

/// RFC 9581 Figure 4, example 2:
/// `1001({1: 1697724754, -6: 873294, -7: {1: 0, -3: 1}})`
///
/// Same timestamp, uncertainty as 1 millisecond.
#[test]
fn rfc9581_fig4_example2_decode() {
    let bytes = hex!("d903e9a3011a65313952251a000d534e26a201002201");
    let decoded: EtimeMapCbor = from_reader(bytes.as_slice()).unwrap();
    let m = &decoded.0;

    assert_eq!(m.base_time, BaseTime::IntegerSecs(1697724754));
    assert_eq!(m.microseconds, Some(873294));

    let unc = m
        .uncertainty
        .as_ref()
        .expect("uncertainty should be present");
    assert_eq!(unc.base_time, BaseTime::IntegerSecs(0));
    assert_eq!(unc.milliseconds, Some(1));
}

/// RFC 9581 Figure 4, example 3:
/// `1001({1: 1697724754, -6: 873294, -7: {1: 0.001}})`
///
/// Same timestamp, uncertainty as floating-point seconds.
#[test]
fn rfc9581_fig4_example3_decode() {
    let bytes = hex!("d903e9a3011a65313952251a000d534e26a101fb3f50624dd2f1a9fc");
    let decoded: EtimeMapCbor = from_reader(bytes.as_slice()).unwrap();
    let m = &decoded.0;

    assert_eq!(m.base_time, BaseTime::IntegerSecs(1697724754));
    assert_eq!(m.microseconds, Some(873294));

    let unc = m
        .uncertainty
        .as_ref()
        .expect("uncertainty should be present");
    match &unc.base_time {
        BaseTime::FloatSecs(f) => {
            assert!((f - 0.001).abs() < 1e-15, "expected ~0.001, got {}", f);
        }
        other => panic!("expected FloatSecs, got {:?}", other),
    }
}

/// RFC 9581 Section 3.7:
/// `1001({1: 851042397, -10: "America/Los_Angeles", -11: {"u-ca": "hebrew"}})`
///
/// Represents 1996-12-19T16:39:57-08:00[America/Los_Angeles][u-ca=hebrew].
#[test]
fn rfc9581_section3_7_ixdtf_decode() {
    let bytes = hex!(
        "d903e9a3011a32b9e05d2973416d65726963612f4c6f735f416e67656c6573"
        "2aa164752d636166686562726577"
    );
    let decoded: EtimeMapCbor = from_reader(bytes.as_slice()).unwrap();
    let m = &decoded.0;

    assert_eq!(m.base_time, BaseTime::IntegerSecs(851042397));
    assert_eq!(m.tz_hint_elective.as_deref(), Some("America/Los_Angeles"));
    let suffix = m
        .suffix_elective
        .as_ref()
        .expect("suffix should be present");
    assert_eq!(
        suffix.get("u-ca"),
        Some(&SuffixValues::One("hebrew".to_string()))
    );
}

// ===========================================================================
// RFC 9581 examples — roundtrip (encode then decode, compare to original)
// ===========================================================================

/// Roundtrip RFC Figure 4 example 1.
#[test]
fn rfc9581_fig4_example1_roundtrip() {
    let bytes = hex!("d903e9a3011a65313952251a000d534e26a20100251903e8");
    let decoded: EtimeMapCbor = from_reader(bytes.as_slice()).unwrap();

    let mut buf = vec![];
    into_writer(&decoded, &mut buf).unwrap();
    let redecoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, redecoded.0);
}

/// Roundtrip RFC Section 3.7 IXDTF example.
#[test]
fn rfc9581_section3_7_roundtrip() {
    let bytes = hex!(
        "d903e9a3011a32b9e05d2973416d65726963612f4c6f735f416e67656c6573"
        "2aa164752d636166686562726577"
    );
    let decoded: EtimeMapCbor = from_reader(bytes.as_slice()).unwrap();

    let mut buf = vec![];
    into_writer(&decoded, &mut buf).unwrap();
    let redecoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, redecoded.0);
}

// ===========================================================================
// Self-constructed roundtrip tests
// ===========================================================================

#[test]
fn etime_integer_secs_roundtrip() {
    let etime = minimal(BaseTime::IntegerSecs(1234567890));
    let tagged: EtimeMapCbor = Required(etime.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, etime);
}

#[test]
fn etime_with_milliseconds_roundtrip() {
    let mut etime = minimal(BaseTime::IntegerSecs(1700000000));
    etime.milliseconds = Some(500);
    let tagged: EtimeMapCbor = Required(etime.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, etime);
}

#[test]
fn etime_with_timescale_and_clock_quality() {
    let mut etime = minimal(BaseTime::IntegerSecs(1700000000));
    etime.timescale_critical = Some(EtimeTimescale::Tai);
    etime.clock_class = Some(6);
    etime.clock_accuracy = Some(0x21);
    etime.offset_scaled_log_variance = Some(0x4e5d);
    let tagged: EtimeMapCbor = Required(etime.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, etime);
}

#[test]
fn etime_with_timezone_roundtrip() {
    let mut etime = minimal(BaseTime::IntegerSecs(1700000000));
    etime.tz_hint_elective = Some("America/New_York".to_string());
    let tagged: EtimeMapCbor = Required(etime.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, etime);
}

#[test]
fn etime_with_uncertainty_roundtrip() {
    let mut etime = minimal(BaseTime::IntegerSecs(1700000000));
    etime.uncertainty = Some(Box::new(minimal(BaseTime::IntegerSecs(0))));
    etime.uncertainty.as_mut().unwrap().milliseconds = Some(5);
    etime.guarantee = Some(Box::new(minimal(BaseTime::IntegerSecs(0))));
    etime.guarantee.as_mut().unwrap().milliseconds = Some(10);
    let tagged: EtimeMapCbor = Required(etime.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, etime);
}

#[test]
fn etime_timescale_conversions() {
    assert_eq!(EtimeTimescale::from(0), EtimeTimescale::Utc);
    assert_eq!(EtimeTimescale::from(1), EtimeTimescale::Tai);
    assert_eq!(EtimeTimescale::from(42), EtimeTimescale::Other(42));
    assert_eq!(u64::from(&EtimeTimescale::Utc), 0);
    assert_eq!(u64::from(&EtimeTimescale::Tai), 1);
    assert_eq!(u64::from(&EtimeTimescale::Other(42)), 42);
}

#[test]
fn duration_tag_1002_roundtrip() {
    let mut dur = minimal(BaseTime::IntegerSecs(3600));
    dur.milliseconds = Some(250);
    let tagged: DurationMapCbor = Required(dur.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: DurationMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, dur);
}

#[test]
fn period_roundtrip() {
    let start = minimal(BaseTime::IntegerSecs(1700000000));
    let end = minimal(BaseTime::IntegerSecs(1700003600));
    let period = Period {
        start: Some(start),
        end: Some(end),
        duration: None,
    };
    let tagged: PeriodCbor = Required(period.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: PeriodCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, period);
}

#[test]
fn period_with_null_start() {
    let end = minimal(BaseTime::IntegerSecs(1700003600));
    let period = Period {
        start: None,
        end: Some(end),
        duration: None,
    };
    let tagged: PeriodCbor = Required(period.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: PeriodCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, period);
}

#[test]
fn etime_with_suffix_info_map_roundtrip() {
    let mut etime = minimal(BaseTime::IntegerSecs(851042397));
    etime.tz_hint_elective = Some("America/Los_Angeles".to_string());
    let mut suffix = BTreeMap::new();
    suffix.insert("u-ca".to_string(), SuffixValues::One("hebrew".to_string()));
    etime.suffix_elective = Some(suffix);
    let tagged: EtimeMapCbor = Required(etime.clone());
    let mut buf = vec![];
    into_writer(&tagged, &mut buf).unwrap();
    let decoded: EtimeMapCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded.0, etime);
}
