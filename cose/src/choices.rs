//! Choice-based structs

// COSE_Messages = COSE_Untagged_Message / COSE_Tagged_Message
//
// COSE_Untagged_Message = COSE_Sign / COSE_Sign1 /
//     COSE_Encrypt / COSE_Encrypt0 /
//     COSE_Mac / COSE_Mac0
//
// COSE_Tagged_Message = COSE_Sign_Tagged / COSE_Sign1_Tagged /
//     COSE_Encrypt_Tagged / COSE_Encrypt0_Tagged /
//     COSE_Mac_Tagged / COSE_Mac0_Tagged

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ciborium::value::Value;
use serde::{Deserialize, Serialize};

//todo enforce header_map or zero size
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
                    Ok(EmptyOrSerializedMap::SerializedMap(v.clone()))
                }
            }
            _ => Err("".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SignatureOrSignature1 {
    Signature,
    Signature1,
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
                "Signature1" => Ok(SignatureOrSignature1::Signature),
                _ => Err("Failed to parse value as Signature or Signature1".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EncStructureContext {
    Encrypt,
    Encrypt0,
    #[serde(rename = "Enc_Recipient")]
    EncRecipient,
    #[serde(rename = "Mac_Recipient")]
    MacRecipient,
    #[serde(rename = "Rec_Recipient")]
    RecRecipient,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum MacStructureContext {
    #[serde(rename = "MAC")]
    Mac,
    #[serde(rename = "MAC0")]
    Mac0,
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
                _ => Err("Failed to parse value as Enc_Structure context".to_string()),
            },
            _ => Err("".to_string()),
        }
    }
}
