//! CBOR-specific definitions
//!
//! In order to support the submodules claim, the Entity Attestation Token (EAT) specification provides
//! a set of CBOR-specific definitions and a set of JSON-specific definitions.
//!
//! The CBOR-specific definitions from [Section 7.3.2](https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#name-cbor-specific-cddl) are below.
//!
//! ```text
//! $EAT-CBOR-Tagged-Token /= CWT-Tagged-Message
//! $EAT-CBOR-Tagged-Token /= BUNDLE-Tagged-Message
//! CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token
//! JSON-Token-Inside-CBOR-Token = tstr
//! CBOR-Nested-Token = JSON-Token-Inside-CBOR-Token / CBOR-Token-Inside-CBOR-Token
//! Nested-Token = CBOR-Nested-Token
//! Submodule = Claims-Set / CBOR-Nested-Token /  Detached-Submodule-Digest
//! ```
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
//! These do not mesh well with the approach of using procedural macros to maintain one structure for JSON use
//! and one structure for CBOR use with TryFrom serving as a bridge (and the structure names distinguished by
//! presence/absence of Cbor suffix). The problem is that while the Submodule definitions share the same components,
//! they are split across different structures. In JSON, the Detached-Submodule-Digest appears in the JSON-Selector type.
//! In CBOR, the Detached-Submodule-Digest appears in the Submodule definition. To harmonize these definitions with the
//! approach taken in this library, the following CDDL is used for CBOR.
//!
//! ```text
//! CBOR-Selector = CBOR-Nested-Token / Detached-Submodule-Digest
//! Submodule = Claims-Set / CBOR-Selector
//! ```
//! To adhere to the naming conventions in this library, JSON-Selector is represented by the Selector enum
//! and CBOR-Selector is represented by the SelectorCbor enum.
//!
use crate::arrays::DetachedSubmoduleDigestCbor;
use crate::json_specific::{JsonSelectorValue, Submodule};
use crate::maps::ClaimsSetClaimsCbor;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::{vec, vec::Vec};
use ciborium::ser::into_writer;
use ciborium::value::Value;
use serde::{Deserialize, Serialize};

// EAT-CBOR-Token = $EAT-CBOR-Tagged-Token / $EAT-CBOR-Untagged-Token
// $EAT-CBOR-Untagged-Token /= CWT-Untagged-Message
// $EAT-CBOR-Untagged-Token /= BUNDLE-Untagged-Message

/// Represents options available for encoding Submodule claims using CBOR except Claims-Set.
///
/// Submodule support for CBOR-encoded EATs relies on the following CDDL definitions:
///
/// ```text
/// $EAT-CBOR-Tagged-Token /= CWT-Tagged-Message
/// $EAT-CBOR-Tagged-Token /= BUNDLE-Tagged-Message
/// CBOR-Token-Inside-CBOR-Token = bstr .cbor $EAT-CBOR-Tagged-Token
/// JSON-Token-Inside-CBOR-Token = tstr
/// CBOR-Nested-Token = JSON-Token-Inside-CBOR-Token / CBOR-Token-Inside-CBOR-Token
/// Nested-Token = CBOR-Nested-Token
/// Submodule = Claims-Set / CBOR-Nested-Token /  Detached-Submodule-Digest
/// ```
/// As noted in the [module documentation](./index.html), the CBOR-specific and JSON-specific definitions do not
/// align well with the naming practices of this library. The following definition is used in lieu
/// of Nested-Token when defining Submodule for CBOR-encoded EATs.
/// ```text
/// CBOR-Selector = CBOR-Nested-Token / Detached-Submodule-Digest
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SelectorCbor {
    JsonTokenInsideCborToken(String),
    CborTokenInsideCborToken(Vec<u8>),
    DetachedSubmoduleDigest(DetachedSubmoduleDigestCbor),
}

/// Represents the options available for encoding Submodule claims using CBOR.
///
/// EAT defines Submodule as below for CBOR-encoded tokens.
/// ```text
/// Submodule = Claims-Set / CBOR-Nested-Token / Detached-Submodule-Digest
/// ```
/// As noted in the [module documentation](./index.html), a modified definition is used in this library
/// to better align with naming and structure management practices.
/// ```text
/// Submodule = Claims-Set / CBOR-Selector
/// ```
/// The SubmoduleEnum uses this alternative definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum SubmoduleCbor {
    //todo consider changing ClaimsSetClaims to be an enum with ClaimsSet accepting duplicates
    ClaimsSet(Box<ClaimsSetClaimsCbor>),
    SelectorCbor(SelectorCbor),
}
impl TryFrom<Value> for SubmoduleCbor {
    type Error = String;
    fn try_from(_value: Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<&Value> for SubmoduleCbor {
    type Error = String;
    fn try_from(_value: &Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<Submodule> for SubmoduleCbor {
    type Error = String;
    fn try_from(value: Submodule) -> Result<Self, Self::Error> {
        match value {
            Submodule::ClaimsSet(b) => {
                let cs = *b;
                //todo unwrap
                let cs_cbor: ClaimsSetClaimsCbor = cs.try_into().unwrap();
                Ok(SubmoduleCbor::ClaimsSet(Box::new(cs_cbor)))
            }
            Submodule::JsonSelector(js) => {
                //todo key off type field instead?
                match &js.nested_token {
                    JsonSelectorValue::JwtMessage(v) => Ok(SubmoduleCbor::SelectorCbor(
                        SelectorCbor::JsonTokenInsideCborToken(v.clone()),
                    )),
                    JsonSelectorValue::CborTokenInsideJsonToken(v) => {
                        //todo unwrap
                        let b = base64::decode(v).unwrap();
                        Ok(SubmoduleCbor::SelectorCbor(
                            SelectorCbor::CborTokenInsideCborToken(b),
                        ))
                    }
                    JsonSelectorValue::DetachedEatBundle(deb) => {
                        let mut encoded_token = vec![];
                        // todo error handling
                        let _ = into_writer(&deb, &mut encoded_token);
                        Ok(SubmoduleCbor::SelectorCbor(
                            SelectorCbor::CborTokenInsideCborToken(encoded_token),
                        ))
                    }
                    JsonSelectorValue::DetachedSubmoduleDigest(v) => {
                        //todo unwrap
                        Ok(SubmoduleCbor::SelectorCbor(
                            SelectorCbor::DetachedSubmoduleDigest(v.try_into().unwrap()),
                        ))
                    }
                }
            }
        }
    }
}
impl TryFrom<&Submodule> for SubmoduleCbor {
    type Error = String;
    fn try_from(value: &Submodule) -> Result<Self, Self::Error> {
        match value {
            Submodule::ClaimsSet(b) => {
                let cs = &**b;
                //todo unwrap
                let cs_cbor: ClaimsSetClaimsCbor = cs.try_into().unwrap();
                Ok(SubmoduleCbor::ClaimsSet(Box::new(cs_cbor)))
            }
            Submodule::JsonSelector(js) => {
                //todo key off type field instead?
                match &js.nested_token {
                    JsonSelectorValue::JwtMessage(v) => Ok(SubmoduleCbor::SelectorCbor(
                        SelectorCbor::JsonTokenInsideCborToken(v.clone()),
                    )),
                    JsonSelectorValue::CborTokenInsideJsonToken(v) => {
                        //todo unwrap
                        let b = base64::decode(v).unwrap();
                        Ok(SubmoduleCbor::SelectorCbor(
                            SelectorCbor::CborTokenInsideCborToken(b),
                        ))
                    }
                    JsonSelectorValue::DetachedEatBundle(deb) => {
                        let mut encoded_token = vec![];
                        // todo error handling
                        let _ = into_writer(&deb, &mut encoded_token);
                        Ok(SubmoduleCbor::SelectorCbor(
                            SelectorCbor::CborTokenInsideCborToken(encoded_token),
                        ))
                    }
                    JsonSelectorValue::DetachedSubmoduleDigest(v) => {
                        //todo unwrap
                        Ok(SubmoduleCbor::SelectorCbor(
                            SelectorCbor::DetachedSubmoduleDigest(v.try_into().unwrap()),
                        ))
                    }
                }
            }
        }
    }
}
