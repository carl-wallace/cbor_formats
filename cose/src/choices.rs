//! Choice-based types from the COSE specification ([RFC 9052]).
//!
//! This module implements the following CDDL productions:
//!
//! | CDDL | Rust |
//! |------|------|
//! | `empty_or_serialized_map` | [`EmptyOrSerializedMap`] |
//! | `Sig_structure` context (`"Signature"` / `"Signature1"`) | [`SignatureOrSignature1`] |
//! | `Enc_structure` context | [`EncStructureContext`] |
//! | `MAC_structure` context (`"MAC"` / `"MAC0"`) | [`MacStructureContext`] |
//!
//! [RFC 9052]: https://datatracker.ietf.org/doc/html/rfc9052

// COSE_Messages = COSE_Untagged_Message / COSE_Tagged_Message
//
// COSE_Untagged_Message = COSE_Sign / COSE_Sign1 /
//     COSE_Encrypt / COSE_Encrypt0 /
//     COSE_Mac / COSE_Mac0
//
// COSE_Tagged_Message = COSE_Sign_Tagged / COSE_Sign1_Tagged /
//     COSE_Encrypt_Tagged / COSE_Encrypt0_Tagged /
//     COSE_Mac_Tagged / COSE_Mac0_Tagged

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ciborium::value::Value;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::maps::HeaderMapCbor;

/// CBOR and JSON encoding/decoding of `empty_or_serialized_map`, see [COSE Section 3].
///
/// ```text
/// empty_or_serialized_map = bstr .cbor header_map / bstr .size 0
/// ```
/// [COSE Section 3]: https://datatracker.ietf.org/doc/html/rfc9052#section-3
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EmptyOrSerializedMap {
    #[serde(with = "serde_bytes")]
    SerializedMap(Vec<u8>),
    #[serde(with = "serde_bytes")]
    Empty(Vec<u8>),
}
impl TryFrom<Value> for EmptyOrSerializedMap {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        EmptyOrSerializedMap::try_from(&value)
    }
}
impl TryFrom<&Value> for EmptyOrSerializedMap {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(v) => {
                if v.is_empty() {
                    Ok(EmptyOrSerializedMap::Empty(v.clone()))
                } else {
                    // Validate that the bytes decode as a valid header_map
                    let _: HeaderMapCbor =
                        ciborium::de::from_reader(v.as_slice()).map_err(|e| {
                            format!("SerializedMap bytes are not a valid header_map: {e}")
                        })?;
                    Ok(EmptyOrSerializedMap::SerializedMap(v.clone()))
                }
            }
            _ => Err("Expected bytes for empty_or_serialized_map".to_string()),
        }
    }
}

/// Context string identifying a `Signature` or `Signature1` operation, used in `Sig_structure` per RFC 9052 Section 4.4.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum SignatureOrSignature1 {
    Signature,
    Signature1,
}
impl Serialize for SignatureOrSignature1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Signature => serializer.serialize_str("Signature"),
            Self::Signature1 => serializer.serialize_str("Signature1"),
        }
    }
}
impl<'de> Deserialize<'de> for SignatureOrSignature1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Signature" => Ok(Self::Signature),
            "Signature1" => Ok(Self::Signature1),
            _ => Err(de::Error::custom("expected Signature or Signature1")),
        }
    }
}
impl TryFrom<Value> for SignatureOrSignature1 {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        SignatureOrSignature1::try_from(&value)
    }
}
impl TryFrom<&Value> for SignatureOrSignature1 {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(t) => match t.as_str() {
                "Signature" => Ok(SignatureOrSignature1::Signature),
                "Signature1" => Ok(SignatureOrSignature1::Signature1),
                _ => Err("Failed to parse value as Signature or Signature1".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl From<&SignatureOrSignature1> for Value {
    fn from(value: &SignatureOrSignature1) -> Self {
        match value {
            SignatureOrSignature1::Signature => Value::Text("Signature".to_string()),
            SignatureOrSignature1::Signature1 => Value::Text("Signature1".to_string()),
        }
    }
}

/// Context string for `Enc_structure`, identifying the encryption operation type per RFC 9052 Section 5.3.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum EncStructureContext {
    Encrypt,
    Encrypt0,
    EncRecipient,
    MacRecipient,
    RecRecipient,
}
impl Serialize for EncStructureContext {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Encrypt => serializer.serialize_str("Encrypt"),
            Self::Encrypt0 => serializer.serialize_str("Encrypt0"),
            Self::EncRecipient => serializer.serialize_str("Enc_Recipient"),
            Self::MacRecipient => serializer.serialize_str("Mac_Recipient"),
            Self::RecRecipient => serializer.serialize_str("Rec_Recipient"),
        }
    }
}
impl<'de> Deserialize<'de> for EncStructureContext {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Encrypt" => Ok(Self::Encrypt),
            "Encrypt0" => Ok(Self::Encrypt0),
            "Enc_Recipient" => Ok(Self::EncRecipient),
            "Mac_Recipient" => Ok(Self::MacRecipient),
            "Rec_Recipient" => Ok(Self::RecRecipient),
            _ => Err(de::Error::custom("expected Enc_structure context")),
        }
    }
}
impl TryFrom<Value> for EncStructureContext {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        EncStructureContext::try_from(&value)
    }
}
impl TryFrom<&Value> for EncStructureContext {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(t) => match t.as_str() {
                "Encrypt" => Ok(EncStructureContext::Encrypt),
                "Encrypt0" => Ok(EncStructureContext::Encrypt0),
                "Enc_Recipient" => Ok(EncStructureContext::EncRecipient),
                "Mac_Recipient" => Ok(EncStructureContext::MacRecipient),
                "Rec_Recipient" => Ok(EncStructureContext::RecRecipient),
                _ => Err("Failed to parse value as Enc_Structure context".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl From<&EncStructureContext> for Value {
    fn from(value: &EncStructureContext) -> Self {
        match value {
            EncStructureContext::Encrypt => Value::Text("Encrypt".to_string()),
            EncStructureContext::Encrypt0 => Value::Text("Encrypt0".to_string()),
            EncStructureContext::EncRecipient => Value::Text("Enc_Recipient".to_string()),
            EncStructureContext::MacRecipient => Value::Text("Mac_Recipient".to_string()),
            EncStructureContext::RecRecipient => Value::Text("Rec_Recipient".to_string()),
        }
    }
}

/// Context string for `MAC_structure`, identifying the MAC operation type per RFC 9052 Section 6.3.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum MacStructureContext {
    Mac,
    Mac0,
}
impl Serialize for MacStructureContext {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mac => serializer.serialize_str("MAC"),
            Self::Mac0 => serializer.serialize_str("MAC0"),
        }
    }
}
impl<'de> Deserialize<'de> for MacStructureContext {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "MAC" => Ok(Self::Mac),
            "MAC0" => Ok(Self::Mac0),
            _ => Err(de::Error::custom("expected MAC or MAC0")),
        }
    }
}
impl TryFrom<Value> for MacStructureContext {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        MacStructureContext::try_from(&value)
    }
}
impl TryFrom<&Value> for MacStructureContext {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(t) => match t.as_str() {
                "MAC" => Ok(MacStructureContext::Mac),
                "MAC0" => Ok(MacStructureContext::Mac0),
                _ => Err("Failed to parse value as MAC_Structure context".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
impl From<&MacStructureContext> for Value {
    fn from(value: &MacStructureContext) -> Self {
        match value {
            MacStructureContext::Mac => Value::Text("MAC".to_string()),
            MacStructureContext::Mac0 => Value::Text("MAC0".to_string()),
        }
    }
}
