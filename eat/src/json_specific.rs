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

use alloc::boxed::Box;
use alloc::string::String;
use core::ops::Deref;

use serde::__private::de::Content;
use serde::{Deserialize, Serialize};

use crate::arrays::{DetachedEatBundle, DetachedSubmoduleDigest};
use crate::cbor_specific::{SelectorCbor, SubmoduleCbor};
use crate::maps::{ClaimsSetClaims, ClaimsSetClaimsCbor};

// EAT-JSON-Token = $EAT-JSON-Token-Formats
//
// $EAT-JSON-Token-Formats /= JWT-Message
// $EAT-JSON-Token-Formats /= BUNDLE-Untagged-Message
//
//
// Nested-Token = JSON-Selector

/// Represents values used to indicate type of nested token in JSON-Selector as defined in [EAT Section 4.2.18].
/// Note, while this enum is extensible the related [JsonSelectorValue](JsonSelectorValue) type is not, at present.
///
/// ```text
/// $JSON-Selector-Type /= "JWT" / "CBOR" / "BUNDLE" / "DIGEST"
/// ```
///
/// [EAT Section 4.2.18]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.2.18
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde_enum_str::Deserialize_enum_str,
    serde_enum_str::Serialize_enum_str,
)]
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
    #[serde(other)]
    Other(String),
}

// todo make JsonSelectorValue and JsonSelector extensible
/// Represents values used to indicate type of nested token in JSON-Selector as defined in [EAT Section 4.2.18]
///
/// ```text
/// $JSON-Selector-Value /= JWT-Message /
///                   CBOR-Token-Inside-JSON-Token /
///                   Detached-EAT-Bundle /
///                   Detached-Submodule-Digest
/// ```
///
/// [EAT Section 4.2.18]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.2.18
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum JsonSelectorValue {
    JwtMessage(String),
    CborTokenInsideJsonToken(String),
    DetachedEatBundle(DetachedEatBundle),
    DetachedSubmoduleDigest(DetachedSubmoduleDigest),
}
impl<'de> serde::Deserialize<'de> for JsonSelectorValue {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        let __content = match <serde::__private::de::Content<'_> as serde::Deserialize>::deserialize(
            __deserializer,
        ) {
            serde::__private::Ok(__val) => __val,
            serde::__private::Err(__err) => {
                return serde::__private::Err(__err);
            }
        };
        match &__content {
            Content::Str(s) => {
                // could use regex crate, but that requires std
                let num = s.matches('.').count();
                if 2 == num || 4 == num {
                    if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                        <String as serde::Deserialize>::deserialize(
                            serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                &__content,
                            ),
                        ),
                        JsonSelectorValue::JwtMessage,
                    ) {
                        return serde::__private::Ok(__ok);
                    }
                }
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <String as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    JsonSelectorValue::CborTokenInsideJsonToken,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            Content::Map(_) => {
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <DetachedEatBundle as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    JsonSelectorValue::DetachedEatBundle,
                ) {
                    return serde::__private::Ok(__ok);
                }
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <DetachedSubmoduleDigest as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    JsonSelectorValue::DetachedSubmoduleDigest,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            _ => {}
        }
        serde::__private::Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum JsonSelectorValue",
        ))
    }
}

/// Provides token_type and nested_token for JSON-encoded submodules
///
/// The `JSON-Selector` array is defined in [EAT Section 4.2.18] and represents a token type and a
/// nested taken values suitable for use in representing a JSON-encoded submodule claim.
///
/// ```text
/// JSON-Selector = [
///    type : $JSON-Selector-Type,
///    nested-token : $JSON-Selector-Value
/// ]
/// ```
/// [SelectorForDeb](SelectorForDeb) is used for DetachedEATBundles.
///
/// [EAT Section 4.2.18]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-eat#section-4.2.18
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct JsonSelector {
    pub token_type: JsonSelectorType,
    pub nested_token: JsonSelectorValue,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum JsonSelectorForDebValue {
    JwtMessage(String),
    CborTokenInsideJsonToken(String),
    DetachedSubmoduleDigest(DetachedSubmoduleDigest),
}
impl<'de> serde::Deserialize<'de> for JsonSelectorForDebValue {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        let __content = match <serde::__private::de::Content<'_> as serde::Deserialize>::deserialize(
            __deserializer,
        ) {
            serde::__private::Ok(__val) => __val,
            serde::__private::Err(__err) => {
                return serde::__private::Err(__err);
            }
        };
        match &__content {
            Content::Str(s) => {
                // could use regex crate, but that requires std
                let num = s.matches('.').count();
                if 2 == num || 4 == num {
                    if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                        <String as serde::Deserialize>::deserialize(
                            serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                &__content,
                            ),
                        ),
                        JsonSelectorForDebValue::JwtMessage,
                    ) {
                        return serde::__private::Ok(__ok);
                    }
                }
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <String as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    JsonSelectorForDebValue::CborTokenInsideJsonToken,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            Content::Map(_) => {
                if let serde::__private::Ok(__ok) = serde::__private::Result::map(
                    <DetachedSubmoduleDigest as serde::Deserialize>::deserialize(
                        serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    JsonSelectorForDebValue::DetachedSubmoduleDigest,
                ) {
                    return serde::__private::Ok(__ok);
                }
            }
            _ => {}
        }
        serde::__private::Err(serde::de::Error::custom(
            "data did not match any variant of untagged enum JsonSelectorValue",
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
