//! Choice-based types from [RFC 9581].
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `$$ETIME-BASETIME` | [`BaseTime`] |
//! | `$ETIME-TIMESCALE` | [`EtimeTimescale`] |
//! | `time-zone-info` | [`TimeZoneInfo`] |
//! | `suffix-info-map` | [`SuffixInfoMap`] |
//! | `~time/~duration` | [`TimeOrDuration`] (alias for [`maps::EtimeMap`](crate::maps::EtimeMap)) |
//!
//! [RFC 9581]: https://www.rfc-editor.org/rfc/rfc9581.html

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Base time value, corresponding to the `$$ETIME-BASETIME` socket in [RFC 9581 Section 3].
///
/// Exactly one base time key (1, 4, or 5) MUST be present in an extended time map.
///
/// ```text
/// $$ETIME-BASETIME //= (1: ~time)      ; POSIX time as integer or float
/// $$ETIME-BASETIME //= (4: ~decfrac)   ; decimal fraction [e10, m]
/// $$ETIME-BASETIME //= (5: ~bigfloat)  ; bigfloat [e2, m]
/// ```
///
/// [RFC 9581 Section 3]: https://www.rfc-editor.org/rfc/rfc9581.html#section-3
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BaseTime {
    /// Key 1: POSIX time as integer (seconds since 1970-01-01T00:00Z).
    IntegerSecs(i64),
    /// Key 1: POSIX time as floating point.
    FloatSecs(f64),
    /// Key 4: decimal fraction `[e10, mantissa]`.
    DecFrac(i64, i128),
    /// Key 5: bigfloat `[e2, mantissa]`.
    BigFloat(i64, i128),
}

/// Timescale indicator, corresponding to the `$ETIME-TIMESCALE` socket in [RFC 9581 Section 3.4].
///
/// ```text
/// $ETIME-TIMESCALE /= &(etime-utc: 0)
/// $ETIME-TIMESCALE /= &(etime-tai: 1)
/// ```
///
/// Used with keys 13 (critical), -1 (elective legacy), and -13 (elective).
///
/// [RFC 9581 Section 3.4]: https://www.rfc-editor.org/rfc/rfc9581.html#section-3.4
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EtimeTimescale {
    /// UTC with POSIX epoch (value 0, default if no timescale key is present).
    Utc,
    /// TAI with PTP epoch (value 1).
    Tai,
    /// Future-registered timescale value.
    Other(u64),
}

impl From<u64> for EtimeTimescale {
    fn from(v: u64) -> Self {
        match v {
            0 => Self::Utc,
            1 => Self::Tai,
            n => Self::Other(n),
        }
    }
}

impl From<&EtimeTimescale> for u64 {
    fn from(v: &EtimeTimescale) -> Self {
        match v {
            EtimeTimescale::Utc => 0,
            EtimeTimescale::Tai => 1,
            EtimeTimescale::Other(n) => *n,
        }
    }
}

/// Time zone information, corresponding to `time-zone-info` in [RFC 9581 Section 3.5].
///
/// A text string matching either a time-zone-name (e.g., `America/Los_Angeles`)
/// or a numeric offset (e.g., `+05:30`), per [RFC 9557].
///
/// Used with keys 10 (critical) and -10 (elective).
///
/// [RFC 9581 Section 3.5]: https://www.rfc-editor.org/rfc/rfc9581.html#section-3.5
/// [RFC 9557]: https://www.rfc-editor.org/rfc/rfc9557.html
pub type TimeZoneInfo = String;

/// Suffix information map, corresponding to `suffix-info-map` in [RFC 9581 Section 3.6].
///
/// A map from suffix keys to one or more suffix values, per [RFC 9557] IXDTF syntax.
///
/// ```text
/// suffix-info-map = { * suffix-key => suffix-values }
/// suffix-values = one-or-more<suffix-value>
/// ```
///
/// Used with keys 11 (critical) and -11 (elective).
///
/// [RFC 9581 Section 3.6]: https://www.rfc-editor.org/rfc/rfc9581.html#section-3.6
/// [RFC 9557]: https://www.rfc-editor.org/rfc/rfc9557.html
pub type SuffixInfoMap = BTreeMap<String, SuffixValues>;

/// One or more suffix values.
///
/// ```text
/// suffix-values = one-or-more<suffix-value>
/// one-or-more<T> = T / [ 2* T ]
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuffixValues {
    /// A single suffix value.
    One(String),
    /// Two or more suffix values.
    More(Vec<String>),
}

/// Value for keys -7 (Uncertainty) and -8 (Guarantee), which accept
/// an unwrapped time or duration map (`~time / ~duration`).
///
/// Since `time` and `duration` share the same map structure (differing only
/// in the outer tag 1001 vs 1002), the unwrapped content is an [`EtimeMap`](crate::maps::EtimeMap)
/// in both cases.
pub type TimeOrDuration = crate::maps::EtimeMap;
