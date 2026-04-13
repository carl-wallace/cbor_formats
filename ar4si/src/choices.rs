//! Choice types from the Attestation Results for Secure Interactions spec
//! ([draft-ietf-rats-ar4si-09]).
//!
//! | CDDL | Rust |
//! |------|------|
//! | `trustworthiness-tier` | [`TrustworthinessTier`] |
//!
//! [draft-ietf-rats-ar4si-09]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09

use alloc::{
    format,
    string::{String, ToString},
};

use ciborium::value::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents the `trustworthiness-tier` choice type from [AR4SI Section 2.3].
///
/// ```text
/// trustworthiness-tier /= JC<"none", 0>
/// trustworthiness-tier /= JC<"affirming", 2>
/// trustworthiness-tier /= JC<"warning", 32>
/// trustworthiness-tier /= JC<"contraindicated", 96>
/// ```
///
/// [AR4SI Section 2.3]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09#section-2.3
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
#[allow(missing_docs)]
pub enum TrustworthinessTier {
    None = 0,
    Affirming = 2,
    Warning = 32,
    Contraindicated = 96,
}

impl TryFrom<&Value> for TrustworthinessTier {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => {
                let val: i8 = (*i).try_into().map_err(|_| {
                    "Integer value out of range for TrustworthinessTier".to_string()
                })?;
                match val {
                    0 => Ok(Self::None),
                    2 => Ok(Self::Affirming),
                    32 => Ok(Self::Warning),
                    96 => Ok(Self::Contraindicated),
                    _ => Err(format!("Unknown TrustworthinessTier value: {}", val)),
                }
            }
            _ => Err("Failed to parse value as a TrustworthinessTier".to_string()),
        }
    }
}

impl TryFrom<Value> for TrustworthinessTier {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
