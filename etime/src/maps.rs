//! Map types from [RFC 9581].
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `etime = #6.1001(...)` | [`EtimeMap`] / [`EtimeMapCbor`] |
//! | `duration = #6.1002(...)` | [`DurationMap`] / [`DurationMapCbor`] |
//! | `period = #6.1003(...)` | [`PeriodCbor`] |
//!
//! Rather than using the RFC's permissive `{* (int/tstr) => any}`, these types
//! enumerate the assigned map keys from the IANA "Time Tag Map Keys" registry,
//! with a catch-all for unrecognized integer keys.
//!
//! [RFC 9581]: https://www.rfc-editor.org/rfc/rfc9581.html

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use ciborium::{
    tag::Required,
    value::{Integer, Value},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser::SerializeMap};

use crate::choices::{BaseTime, EtimeTimescale, SuffixInfoMap, SuffixValues, TimeZoneInfo};

/// Extended time map containing assigned keys from the IANA "Time Tag Map Keys"
/// registry defined in [RFC 9581].
///
/// The CDDL in the RFC uses a permissive `{* (int/tstr) => any}` for the tagged
/// form. This struct instead enumerates all assigned keys for type safety:
///
/// ```text
/// etime-assigned-map-keys = {
///   ; exactly one unsigned integer key MUST be present to specify the "base time"
///   ? 1 => ~time              ; base time value as in CBOR tag 1
///   ? 4 => ~decfrac           ; base time value as in CBOR tag 4
///   ? 5 => ~bigfloat          ; base time value as in CBOR tag 5
///
///   ; other (non-base time) unsigned integer keys representing critical supplementary
///   ; information MAY be present
///   ? 10 => time-zone-info    ; IXDTF Time Zone Hint (critical)
///   ? 11 => suffix-info-map   ; IXDTF Suffix Information (critical)
///   ? 13 => $ETIME-TIMESCALE  ; timescale (critical)
///
///   ; zero or more negative values MAY be present. if none of the $ETIME-TIMESCALE keys are
///   ; present (i.e., 13, -1 and -13 at present), the default timescale value 0 is implied.
///   ? -1 => $ETIME-TIMESCALE  ; timescale (elective) legacy
///   ? -2 => uint .size 1      ; Clock Class (RFC8575)
///   ? -3 => uint              ; milliseconds
///   ? -4 => uint .size 1      ; Clock Accuracy (RFC8575)
///   ? -5 => uint .size 2      ; Offset-Scaled Log Variance (RFC8575)
///   ? -6 => uint              ; microseconds
///   ? -7 => ~time/~duration   ; Uncertainty
///   ? -8 => ~time/~duration   ; Guarantee
///   ? -9 => uint              ; nanoseconds
///   ? -10 => time-zone-info   ; IXDTF Time Zone Hint (elective)
///   ? -11 => suffix-info-map  ; IXDTF Suffix Information (elective)
///   ? -12 => uint             ; picoseconds
///   ? -13 => $ETIME-TIMESCALE ; timescale (elective)
///   ? -15 => uint             ; femtoseconds
///   ? -18 => uint             ; attoseconds
///
///   ; keys not assigned in the "Time Tag Map Keys" IANA registry may be defined in the
///   ; future (specification required)
///   * int => any
/// }
/// ```
///
/// [RFC 9581]: https://www.rfc-editor.org/rfc/rfc9581.html
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct EtimeMap {
    // --- base time (exactly one MUST be present) ---
    pub base_time: BaseTime,

    // --- critical supplementary information (unsigned integer keys) ---
    /// Key 10: IXDTF Time Zone Hint (critical).
    pub tz_hint_critical: Option<TimeZoneInfo>,
    /// Key 11: IXDTF Suffix Information (critical).
    pub suffix_critical: Option<SuffixInfoMap>,
    /// Key 13: timescale (critical).
    pub timescale_critical: Option<EtimeTimescale>,

    // --- elective supplementary information (negative integer keys) ---
    /// Key -1: timescale (elective, legacy).
    pub timescale_legacy: Option<EtimeTimescale>,
    /// Key -2: Clock Class per RFC 8575.
    pub clock_class: Option<u8>,
    /// Key -3: fractional seconds in milliseconds.
    pub milliseconds: Option<u64>,
    /// Key -4: Clock Accuracy per RFC 8575.
    pub clock_accuracy: Option<u8>,
    /// Key -5: Offset-Scaled Log Variance per RFC 8575.
    pub offset_scaled_log_variance: Option<u16>,
    /// Key -6: fractional seconds in microseconds.
    pub microseconds: Option<u64>,
    /// Key -7: Uncertainty (`~time / ~duration`, i.e., an unwrapped etime map).
    pub uncertainty: Option<Box<EtimeMap>>,
    /// Key -8: Guarantee (`~time / ~duration`, i.e., an unwrapped etime map).
    pub guarantee: Option<Box<EtimeMap>>,
    /// Key -9: fractional seconds in nanoseconds.
    pub nanoseconds: Option<u64>,
    /// Key -10: IXDTF Time Zone Hint (elective).
    pub tz_hint_elective: Option<TimeZoneInfo>,
    /// Key -11: IXDTF Suffix Information (elective).
    pub suffix_elective: Option<SuffixInfoMap>,
    /// Key -12: fractional seconds in picoseconds.
    pub picoseconds: Option<u64>,
    /// Key -13: timescale (elective).
    pub timescale_elective: Option<EtimeTimescale>,
    /// Key -15: fractional seconds in femtoseconds.
    pub femtoseconds: Option<u64>,
    /// Key -18: fractional seconds in attoseconds.
    pub attoseconds: Option<u64>,

    // --- unrecognized keys ---
    /// Unrecognized integer-keyed entries (for forward compatibility).
    pub other: Vec<(i64, Value)>,
}

impl EtimeMap {
    /// Validates that the map satisfies RFC 9581 constraints.
    pub fn validate(&self) -> Result<(), String> {
        // Clock Class must fit in 1 byte (enforced by u8 type)
        // Clock Accuracy must fit in 1 byte (enforced by u8 type)
        // Offset-Scaled Log Variance must fit in 2 bytes (enforced by u16 type)

        // At most one timescale should typically be present, but the spec allows
        // critical + elective to coexist (the critical value takes precedence).
        Ok(())
    }
}

/// CBOR-encoded form of [`EtimeMap`], wrapped in tag 1001.
///
/// ```text
/// etime = #6.1001({* (int/tstr) => any})
/// ```
pub type EtimeMapCbor = Required<EtimeMap, 1001>;

/// Duration map, structurally identical to [`EtimeMap`] but wrapped in tag 1002.
///
/// ```text
/// duration = #6.1002({* (int/tstr) => any})
/// ```
pub type DurationMap = EtimeMap;

/// CBOR-encoded form of [`DurationMap`], wrapped in tag 1002.
pub type DurationMapCbor = Required<DurationMap, 1002>;

/// Period type, wrapped in tag 1003.
///
/// ```text
/// period = #6.1003([~etime/null, ~etime/null, ?~duration])
/// ```
///
/// A period is an array of two optional time points and an optional duration.
#[derive(Clone, Debug, PartialEq)]
pub struct Period {
    /// Start time (null if unbounded).
    pub start: Option<EtimeMap>,
    /// End time (null if unbounded).
    pub end: Option<EtimeMap>,
    /// Duration (optional).
    pub duration: Option<EtimeMap>,
}

/// CBOR-encoded form of [`Period`], wrapped in tag 1003.
pub type PeriodCbor = Required<Period, 1003>;

// ---------------------------------------------------------------------------
// EtimeMap Serialize
// ---------------------------------------------------------------------------

impl Serialize for EtimeMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Count entries
        let mut count = 1; // base time
        if self.tz_hint_critical.is_some() {
            count += 1;
        }
        if self.suffix_critical.is_some() {
            count += 1;
        }
        if self.timescale_critical.is_some() {
            count += 1;
        }
        if self.timescale_legacy.is_some() {
            count += 1;
        }
        if self.clock_class.is_some() {
            count += 1;
        }
        if self.milliseconds.is_some() {
            count += 1;
        }
        if self.clock_accuracy.is_some() {
            count += 1;
        }
        if self.offset_scaled_log_variance.is_some() {
            count += 1;
        }
        if self.microseconds.is_some() {
            count += 1;
        }
        if self.uncertainty.is_some() {
            count += 1;
        }
        if self.guarantee.is_some() {
            count += 1;
        }
        if self.nanoseconds.is_some() {
            count += 1;
        }
        if self.tz_hint_elective.is_some() {
            count += 1;
        }
        if self.suffix_elective.is_some() {
            count += 1;
        }
        if self.picoseconds.is_some() {
            count += 1;
        }
        if self.timescale_elective.is_some() {
            count += 1;
        }
        if self.femtoseconds.is_some() {
            count += 1;
        }
        if self.attoseconds.is_some() {
            count += 1;
        }
        count += self.other.len();

        let mut map = serializer.serialize_map(Some(count))?;

        // Base time
        match &self.base_time {
            BaseTime::IntegerSecs(v) => {
                map.serialize_entry(&1, v)?;
            }
            BaseTime::FloatSecs(v) => {
                map.serialize_entry(&1, v)?;
            }
            BaseTime::DecFrac(e, m) => {
                map.serialize_entry(
                    &4,
                    &Value::Array(vec![Value::Integer(Integer::from(*e)), value_from_i128(*m)]),
                )?;
            }
            BaseTime::BigFloat(e, m) => {
                map.serialize_entry(
                    &5,
                    &Value::Array(vec![Value::Integer(Integer::from(*e)), value_from_i128(*m)]),
                )?;
            }
        }

        // Critical supplementary
        if let Some(ref tz) = self.tz_hint_critical {
            map.serialize_entry(&10, tz)?;
        }
        if let Some(ref sf) = self.suffix_critical {
            map.serialize_entry(&11, &suffix_to_value(sf))?;
        }
        if let Some(ref ts) = self.timescale_critical {
            map.serialize_entry(&13, &u64::from(ts))?;
        }

        // Elective supplementary
        if let Some(ref ts) = self.timescale_legacy {
            map.serialize_entry(&-1, &u64::from(ts))?;
        }
        if let Some(v) = self.clock_class {
            map.serialize_entry(&-2, &v)?;
        }
        if let Some(v) = self.milliseconds {
            map.serialize_entry(&-3, &v)?;
        }
        if let Some(v) = self.clock_accuracy {
            map.serialize_entry(&-4, &v)?;
        }
        if let Some(v) = self.offset_scaled_log_variance {
            map.serialize_entry(&-5, &v)?;
        }
        if let Some(v) = self.microseconds {
            map.serialize_entry(&-6, &v)?;
        }
        if let Some(ref td) = self.uncertainty {
            map.serialize_entry(&-7, td.as_ref())?;
        }
        if let Some(ref td) = self.guarantee {
            map.serialize_entry(&-8, td.as_ref())?;
        }
        if let Some(v) = self.nanoseconds {
            map.serialize_entry(&-9, &v)?;
        }
        if let Some(ref tz) = self.tz_hint_elective {
            map.serialize_entry(&-10, tz)?;
        }
        if let Some(ref sf) = self.suffix_elective {
            map.serialize_entry(&-11, &suffix_to_value(sf))?;
        }
        if let Some(v) = self.picoseconds {
            map.serialize_entry(&-12, &v)?;
        }
        if let Some(ref ts) = self.timescale_elective {
            map.serialize_entry(&-13, &u64::from(ts))?;
        }
        if let Some(v) = self.femtoseconds {
            map.serialize_entry(&-15, &v)?;
        }
        if let Some(v) = self.attoseconds {
            map.serialize_entry(&-18, &v)?;
        }

        // Unrecognized keys
        for (k, v) in &self.other {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}

fn value_from_i128(v: i128) -> Value {
    // ciborium Integer can hold i128
    Value::Integer(Integer::try_from(v).unwrap_or_else(|_| Integer::from(0)))
}

fn suffix_to_value(map: &SuffixInfoMap) -> Value {
    let entries: Vec<(Value, Value)> = map
        .iter()
        .map(|(k, v)| {
            let key = Value::Text(k.clone());
            let val = match v {
                SuffixValues::One(s) => Value::Text(s.clone()),
                SuffixValues::More(arr) => {
                    Value::Array(arr.iter().map(|s| Value::Text(s.clone())).collect())
                }
            };
            (key, val)
        })
        .collect();
    Value::Map(entries)
}

// ---------------------------------------------------------------------------
// EtimeMap Deserialize
// ---------------------------------------------------------------------------

impl<'de> Deserialize<'de> for EtimeMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Deserialize as a ciborium Value first, then extract fields
        let value = Value::deserialize(deserializer)?;
        let entries = match &value {
            Value::Map(m) => m,
            _ => return Err(de::Error::custom("expected a CBOR map for EtimeMap")),
        };

        let mut base_time: Option<BaseTime> = None;
        let mut tz_hint_critical = None;
        let mut suffix_critical = None;
        let mut timescale_critical = None;
        let mut timescale_legacy = None;
        let mut clock_class = None;
        let mut milliseconds = None;
        let mut clock_accuracy = None;
        let mut offset_scaled_log_variance = None;
        let mut microseconds = None;
        let mut uncertainty = None;
        let mut guarantee = None;
        let mut nanoseconds = None;
        let mut tz_hint_elective = None;
        let mut suffix_elective = None;
        let mut picoseconds = None;
        let mut timescale_elective = None;
        let mut femtoseconds = None;
        let mut attoseconds = None;
        let mut other = Vec::new();

        for (k, v) in entries {
            let key = match k {
                Value::Integer(i) => {
                    let val: i128 = (*i).into();
                    match i64::try_from(val) {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(de::Error::custom("integer key out of i64 range"));
                        }
                    }
                }
                _ => {
                    // text or other keys: skip for now (could store in a separate field)
                    continue;
                }
            };

            match key {
                1 => {
                    base_time = Some(parse_base_time_1(v).map_err(de::Error::custom)?);
                }
                4 => {
                    base_time = Some(parse_decfrac(v).map_err(de::Error::custom)?);
                }
                5 => {
                    base_time = Some(parse_bigfloat(v).map_err(de::Error::custom)?);
                }
                10 => {
                    tz_hint_critical = Some(parse_text(v).map_err(de::Error::custom)?);
                }
                11 => {
                    suffix_critical = Some(parse_suffix_info_map(v).map_err(de::Error::custom)?);
                }
                13 => {
                    timescale_critical = Some(parse_timescale(v).map_err(de::Error::custom)?);
                }
                -1 => {
                    timescale_legacy = Some(parse_timescale(v).map_err(de::Error::custom)?);
                }
                -2 => {
                    clock_class = Some(parse_u8(v).map_err(de::Error::custom)?);
                }
                -3 => {
                    milliseconds = Some(parse_u64(v).map_err(de::Error::custom)?);
                }
                -4 => {
                    clock_accuracy = Some(parse_u8(v).map_err(de::Error::custom)?);
                }
                -5 => {
                    offset_scaled_log_variance = Some(parse_u16(v).map_err(de::Error::custom)?);
                }
                -6 => {
                    microseconds = Some(parse_u64(v).map_err(de::Error::custom)?);
                }
                -7 => {
                    uncertainty = Some(Box::new(
                        parse_etime_from_value(v).map_err(de::Error::custom)?,
                    ));
                }
                -8 => {
                    guarantee = Some(Box::new(
                        parse_etime_from_value(v).map_err(de::Error::custom)?,
                    ));
                }
                -9 => {
                    nanoseconds = Some(parse_u64(v).map_err(de::Error::custom)?);
                }
                -10 => {
                    tz_hint_elective = Some(parse_text(v).map_err(de::Error::custom)?);
                }
                -11 => {
                    suffix_elective = Some(parse_suffix_info_map(v).map_err(de::Error::custom)?);
                }
                -12 => {
                    picoseconds = Some(parse_u64(v).map_err(de::Error::custom)?);
                }
                -13 => {
                    timescale_elective = Some(parse_timescale(v).map_err(de::Error::custom)?);
                }
                -15 => {
                    femtoseconds = Some(parse_u64(v).map_err(de::Error::custom)?);
                }
                -18 => {
                    attoseconds = Some(parse_u64(v).map_err(de::Error::custom)?);
                }
                _ => {
                    other.push((key, v.clone()));
                }
            }
        }

        let base_time =
            base_time.ok_or_else(|| de::Error::custom("missing base time key (1, 4, or 5)"))?;

        Ok(EtimeMap {
            base_time,
            tz_hint_critical,
            suffix_critical,
            timescale_critical,
            timescale_legacy,
            clock_class,
            milliseconds,
            clock_accuracy,
            offset_scaled_log_variance,
            microseconds,
            uncertainty,
            guarantee,
            nanoseconds,
            tz_hint_elective,
            suffix_elective,
            picoseconds,
            timescale_elective,
            femtoseconds,
            attoseconds,
            other,
        })
    }
}

// ---------------------------------------------------------------------------
// Period Serialize / Deserialize
// ---------------------------------------------------------------------------

impl Serialize for Period {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let len = if self.duration.is_some() { 3 } else { 2 };
        let mut seq = serializer.serialize_seq(Some(len))?;

        match &self.start {
            Some(m) => seq.serialize_element(&Required::<&EtimeMap, 1001>(m))?,
            None => seq.serialize_element(&Value::Null)?,
        }
        match &self.end {
            Some(m) => seq.serialize_element(&Required::<&EtimeMap, 1001>(m))?,
            None => seq.serialize_element(&Value::Null)?,
        }
        if let Some(ref d) = self.duration {
            seq.serialize_element(&Required::<&EtimeMap, 1002>(d))?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for Period {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let arr = match &value {
            Value::Array(a) => a,
            _ => return Err(de::Error::custom("expected array for Period")),
        };

        if arr.len() < 2 || arr.len() > 3 {
            return Err(de::Error::custom("Period array must have 2 or 3 elements"));
        }

        let start = parse_optional_tagged_etime(&arr[0], 1001).map_err(de::Error::custom)?;
        let end = parse_optional_tagged_etime(&arr[1], 1001).map_err(de::Error::custom)?;
        let duration = if arr.len() == 3 {
            parse_optional_tagged_etime(&arr[2], 1002).map_err(de::Error::custom)?
        } else {
            None
        };

        Ok(Period {
            start,
            end,
            duration,
        })
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

fn parse_base_time_1(v: &Value) -> Result<BaseTime, String> {
    match v {
        Value::Integer(i) => {
            let val: i128 = (*i).into();
            i64::try_from(val)
                .map(BaseTime::IntegerSecs)
                .map_err(|_| "base time integer out of i64 range".to_string())
        }
        Value::Float(f) => Ok(BaseTime::FloatSecs(*f)),
        _ => Err("expected integer or float for base time key 1".to_string()),
    }
}

fn parse_decfrac(v: &Value) -> Result<BaseTime, String> {
    match v {
        Value::Array(arr) if arr.len() == 2 => {
            let e = parse_i64(&arr[0])?;
            let m = parse_i128(&arr[1])?;
            Ok(BaseTime::DecFrac(e, m))
        }
        _ => Err("expected [exponent, mantissa] array for decfrac (key 4)".to_string()),
    }
}

fn parse_bigfloat(v: &Value) -> Result<BaseTime, String> {
    match v {
        Value::Array(arr) if arr.len() == 2 => {
            let e = parse_i64(&arr[0])?;
            let m = parse_i128(&arr[1])?;
            Ok(BaseTime::BigFloat(e, m))
        }
        _ => Err("expected [exponent, mantissa] array for bigfloat (key 5)".to_string()),
    }
}

fn parse_timescale(v: &Value) -> Result<EtimeTimescale, String> {
    parse_u64(v).map(EtimeTimescale::from)
}

fn parse_text(v: &Value) -> Result<String, String> {
    match v {
        Value::Text(s) => Ok(s.clone()),
        _ => Err("expected text string".to_string()),
    }
}

fn parse_suffix_info_map(v: &Value) -> Result<SuffixInfoMap, String> {
    match v {
        Value::Map(entries) => {
            let mut map = SuffixInfoMap::new();
            for (k, val) in entries {
                let key = match k {
                    Value::Text(s) => s.clone(),
                    _ => return Err("suffix-info-map key must be text".to_string()),
                };
                let values = match val {
                    Value::Text(s) => SuffixValues::One(s.clone()),
                    Value::Array(arr) => {
                        let mut v = Vec::new();
                        for item in arr {
                            match item {
                                Value::Text(s) => v.push(s.clone()),
                                _ => {
                                    return Err(
                                        "suffix-info-map array values must be text".to_string()
                                    );
                                }
                            }
                        }
                        SuffixValues::More(v)
                    }
                    _ => return Err("suffix-info-map value must be text or array".to_string()),
                };
                map.insert(key, values);
            }
            Ok(map)
        }
        _ => Err("expected map for suffix-info-map".to_string()),
    }
}

fn parse_u8(v: &Value) -> Result<u8, String> {
    parse_u64(v).and_then(|n| u8::try_from(n).map_err(|_| "value out of u8 range".to_string()))
}

fn parse_u16(v: &Value) -> Result<u16, String> {
    parse_u64(v).and_then(|n| u16::try_from(n).map_err(|_| "value out of u16 range".to_string()))
}

fn parse_u64(v: &Value) -> Result<u64, String> {
    match v {
        Value::Integer(i) => {
            let val: i128 = (*i).into();
            u64::try_from(val).map_err(|_| "expected unsigned integer".to_string())
        }
        _ => Err("expected unsigned integer".to_string()),
    }
}

fn parse_i64(v: &Value) -> Result<i64, String> {
    match v {
        Value::Integer(i) => {
            let val: i128 = (*i).into();
            i64::try_from(val).map_err(|_| "integer out of i64 range".to_string())
        }
        _ => Err("expected integer".to_string()),
    }
}

fn parse_i128(v: &Value) -> Result<i128, String> {
    match v {
        Value::Integer(i) => Ok((*i).into()),
        _ => Err("expected integer".to_string()),
    }
}

fn parse_optional_tagged_etime(v: &Value, tag: u64) -> Result<Option<EtimeMap>, String> {
    match v {
        Value::Null => Ok(None),
        Value::Tag(t, inner) if *t == tag => parse_etime_from_value(inner).map(Some),
        _ => Err(alloc::format!(
            "expected null or tag {} for period element",
            tag
        )),
    }
}

fn parse_etime_from_value(v: &Value) -> Result<EtimeMap, String> {
    let entries = match v {
        Value::Map(m) => m,
        _ => return Err("expected map for etime".to_string()),
    };

    let mut base_time: Option<BaseTime> = None;
    let mut tz_hint_critical = None;
    let mut suffix_critical = None;
    let mut timescale_critical = None;
    let mut timescale_legacy = None;
    let mut clock_class = None;
    let mut milliseconds = None;
    let mut clock_accuracy = None;
    let mut offset_scaled_log_variance = None;
    let mut microseconds = None;
    let mut uncertainty = None;
    let mut guarantee = None;
    let mut nanoseconds = None;
    let mut tz_hint_elective = None;
    let mut suffix_elective = None;
    let mut picoseconds = None;
    let mut timescale_elective = None;
    let mut femtoseconds = None;
    let mut attoseconds = None;
    let mut other = Vec::new();

    for (k, val) in entries {
        let key = match k {
            Value::Integer(i) => {
                let v: i128 = (*i).into();
                i64::try_from(v).map_err(|_| "integer key out of i64 range".to_string())?
            }
            _ => continue,
        };

        match key {
            1 => base_time = Some(parse_base_time_1(val)?),
            4 => base_time = Some(parse_decfrac(val)?),
            5 => base_time = Some(parse_bigfloat(val)?),
            10 => tz_hint_critical = Some(parse_text(val)?),
            11 => suffix_critical = Some(parse_suffix_info_map(val)?),
            13 => timescale_critical = Some(parse_timescale(val)?),
            -1 => timescale_legacy = Some(parse_timescale(val)?),
            -2 => clock_class = Some(parse_u8(val)?),
            -3 => milliseconds = Some(parse_u64(val)?),
            -4 => clock_accuracy = Some(parse_u8(val)?),
            -5 => offset_scaled_log_variance = Some(parse_u16(val)?),
            -6 => microseconds = Some(parse_u64(val)?),
            -7 => uncertainty = Some(Box::new(parse_etime_from_value(val)?)),
            -8 => guarantee = Some(Box::new(parse_etime_from_value(val)?)),
            -9 => nanoseconds = Some(parse_u64(val)?),
            -10 => tz_hint_elective = Some(parse_text(val)?),
            -11 => suffix_elective = Some(parse_suffix_info_map(val)?),
            -12 => picoseconds = Some(parse_u64(val)?),
            -13 => timescale_elective = Some(parse_timescale(val)?),
            -15 => femtoseconds = Some(parse_u64(val)?),
            -18 => attoseconds = Some(parse_u64(val)?),
            _ => other.push((key, val.clone())),
        }
    }

    let base_time = base_time.ok_or_else(|| "missing base time key (1, 4, or 5)".to_string())?;

    Ok(EtimeMap {
        base_time,
        tz_hint_critical,
        suffix_critical,
        timescale_critical,
        timescale_legacy,
        clock_class,
        milliseconds,
        clock_accuracy,
        offset_scaled_log_variance,
        microseconds,
        uncertainty,
        guarantee,
        nanoseconds,
        tz_hint_elective,
        suffix_elective,
        picoseconds,
        timescale_elective,
        femtoseconds,
        attoseconds,
        other,
    })
}
