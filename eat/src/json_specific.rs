//! JSON-specific definitions
//!
//! In order to support the submodules claim, the Entity Attestation Token (EAT) specification provides
//! a set of CBOR-specific definitions and a set of JSON-specific definitions.
//!
//! The JSON-specific definitions from [Section 7.3.3](https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#name-json-specific-cddl) are below.
//!
//! ```text
//! $JSON-Selector-Value /= JWT-Message / CBOR-Token-Inside-JSON-Token / Detached-EAT-Bundle / Detached-Submodule-Digest
//!
//! JSON-Selector = [
//!    type : $JSON-Selector-Type,
//!    nested-token : $JSON-Selector-Value
//! ]
//! Submodule = Claims-Set / JSON-Selector
//! ```
//!
//! This module provides support for JSON-encoded Submodule claims. See [cbor_specific](../cbor_specific/index.html) module for
//! details regarding support for CBOR-encoded Submodule claims.

use crate::alloc::string::ToString;
use crate::arrays::{DetachedEatBundle, DetachedSubmoduleDigest};
use crate::maps::ClaimsSetClaims;
use alloc::format;
use alloc::string::String;
use cbor_derive::StructToArray;
use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{__private::size_hint, de::Error, de::Visitor};

use crate::cbor_specific::{SelectorCbor, SubmoduleCbor};
use crate::maps::ClaimsSetClaimsCbor;
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use core::ops::Deref;

// EAT-JSON-Token = $EAT-JSON-Token-Formats
//
// $EAT-JSON-Token-Formats /= JWT-Message
// $EAT-JSON-Token-Formats /= BUNDLE-Untagged-Message
//
//
// Nested-Token = JSON-Selector

// $JSON-Selector-Type /= "JWT" / "CBOR" / "BUNDLE" / "DIGEST"

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum JsonSelectorType {
    #[serde(rename = "JWT")]
    Jwt,
    #[serde(rename = "CBOR")]
    Cbor,
    #[serde(rename = "BUNDLE")]
    Bundle,
    #[serde(rename = "DIGEST")]
    Digest,
    Other(String),
}
impl TryFrom<Value> for JsonSelectorType {
    type Error = String;
    fn try_from(_value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&Value> for JsonSelectorType {
    type Error = String;
    fn try_from(_value: &Value) -> Result<Self, Self::Error> {
        todo!()
    }
}

// ;JWT-Message =
// ;   text .regexp "[A-Za-z0-9_=-]+\.[A-Za-z0-9_=-]+\.[A-Za-z0-9_=-]+"
//
// CBOR-Token-Inside-JSON-Token = base64-url-text
//
// $JSON-Selector-Value /= JWT-Message /
//                   CBOR-Token-Inside-JSON-Token /
//                   Detached-EAT-Bundle /
//                   Detached-Submodule-Digest
//
// JSON-Selector = [
//    type : $JSON-Selector-Type,
//    nested-token : $JSON-Selector-Value
// ]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum JsonSelectorValue {
    JwtMessage(String),
    CborTokenInsideJsonToken(String),
    DetachedEatBundle(DetachedEatBundle),
    DetachedSubmoduleDigest(DetachedSubmoduleDigest),
}
impl TryFrom<Value> for JsonSelectorValue {
    type Error = String;
    fn try_from(_value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&Value> for JsonSelectorValue {
    type Error = String;
    fn try_from(_value: &Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct JsonSelector {
    pub token_type: JsonSelectorType,
    pub nested_token: JsonSelectorValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum JsonSelectorForDebValue {
    JwtMessage(String),
    CborTokenInsideJsonToken(String),
    DetachedSubmoduleDigest(DetachedSubmoduleDigest),
}
impl TryFrom<Value> for JsonSelectorForDebValue {
    type Error = String;
    fn try_from(_value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&Value> for JsonSelectorForDebValue {
    type Error = String;
    fn try_from(_value: &Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct SelectorForDeb {
    pub token_type: JsonSelectorType,
    pub nested_token: JsonSelectorForDebValue,
}

// $$Claims-Set-Claims //= (submods-label => { + text => Submodule })
//
// Submodule = Claims-Set / JSON-Selector
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Submodule {
    //todo consider changing ClaimsSetClaims to be an enum with ClaimsSet accepting duplicates
    ClaimsSet(Box<ClaimsSetClaims>),
    JsonSelector(JsonSelector),
}
impl TryFrom<SubmoduleCbor> for Submodule {
    type Error = String;
    fn try_from(_value: SubmoduleCbor) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&SubmoduleCbor> for Submodule {
    type Error = String;
    fn try_from(value: &SubmoduleCbor) -> Result<Self, Self::Error> {
        match value {
            SubmoduleCbor::ClaimsSet(b) => {
                let cs: &ClaimsSetClaimsCbor = b.deref();
                //todo unwrap
                let cs_json: ClaimsSetClaims = cs.try_into().unwrap();
                Ok(Submodule::ClaimsSet(Box::new(cs_json)))
            }
            SubmoduleCbor::SelectorCbor(SelectorCbor::CborTokenInsideCborToken(b)) => {
                let js = JsonSelector {
                    token_type: JsonSelectorType::Cbor,
                    nested_token: JsonSelectorValue::CborTokenInsideJsonToken(base64::encode(b)),
                };
                Ok(Submodule::JsonSelector(js))
            }
            SubmoduleCbor::SelectorCbor(SelectorCbor::JsonTokenInsideCborToken(s)) => {
                let js = JsonSelector {
                    token_type: JsonSelectorType::Jwt,
                    nested_token: JsonSelectorValue::JwtMessage(s.clone()),
                };
                Ok(Submodule::JsonSelector(js))
            }
            SubmoduleCbor::SelectorCbor(SelectorCbor::DetachedSubmoduleDigest(dsm)) => {
                //todo unwrap
                let js = JsonSelector {
                    token_type: JsonSelectorType::Bundle,
                    nested_token: JsonSelectorValue::DetachedSubmoduleDigest(
                        dsm.try_into().unwrap(),
                    ),
                };
                Ok(Submodule::JsonSelector(js))
            }
        }
    }
}
